use crate::types::PicsFn;
use crate::types::Score;
use crate::types::{Pic, PicSourceId, Pics, ScoresMatrix};
use ndarray::{Array1, Axis};

type ScoreSum = Array1<u32>;

enum NextOption {
    Idx(usize),
    Pic { pic: Pic, other_idx: usize },
    None,
}

trait GreedyNext {
    fn next(&self, pics: &Pics, scores_matrix: &ScoresMatrix) -> NextOption;
    fn score_sum(&self, axis_len: usize, scores_matrix: &ScoresMatrix, pics: &Pics) -> u32;
}

impl GreedyNext for Pic {
    fn score_sum(&self, axis_len: usize, scores_matrix: &ScoresMatrix, pics: &Pics) -> u32 {
        let scores = if self.id < axis_len {
            scores_matrix.row(self.id).to_vec()
        } else {
            self.all_scores(&pics).to_vec()
        };
        let mut score_sum = 0;
        for score in scores {
            score_sum += score as u32;
        }
        score_sum
    }
    fn next(&self, pics: &Pics, scores_matrix: &ScoresMatrix) -> NextOption {
        #[derive(Debug, Clone)]
        struct Best {
            idx: usize,
            pic: Pic,
            score: u8,
            score_sum: u32,
        }
        fn best_contender(
            contenders: Vec<Best>,
            axis_len: usize,
            scores_matrix: &ScoresMatrix,
            pics: &Pics,
        ) -> Best {
            let mut best = contenders[0].clone();
            for contender in contenders {
                let contender_score_sum = contender.pic.score_sum(axis_len, scores_matrix, pics);
                if contender_score_sum < best.score_sum {
                    best = contender.clone();
                    best.score_sum = contender_score_sum;
                }
            }
            best
        }
        fn complete_slide(pic: &Pic, pics: &Pics, scores_matrix: &ScoresMatrix) -> NextOption {
            fn score_pair(a: &Pic, b: &Pic, axis_len: usize, scores_matrix: &ScoresMatrix) -> u8 {
                if a.id < axis_len && b.id < axis_len {
                    scores_matrix[[a.id, b.id]]
                } else {
                    a.score_with(&b)
                }
            }
            let axis_len = scores_matrix.len_of(Axis(0));
            if pics.is_empty() {
                return NextOption::None;
            }
            let best = Best {
                idx: 0,
                pic: pics[0].clone(),
                score: score_pair(&pic, &pics[0], axis_len, scores_matrix),
                score_sum: pics[0].score_sum(axis_len, scores_matrix, pics),
            };
            let mut contenders = vec![best.clone()];
            for (idx, contender) in pics.iter().enumerate() {
                let contender_score = score_pair(&pic, &contender, axis_len, scores_matrix);
                let contender = Best {
                    pic: contender.clone(),
                    idx,
                    score: contender_score,
                    score_sum: 0,
                };
                if contender_score > best.score {
                    contenders.clear();
                    contenders.push(best.clone());
                } else if contender_score == best.score {
                    contenders.push(contender.clone());
                }
            }
            NextOption::Idx(best_contender(contenders, axis_len, scores_matrix, pics).idx)
        }
        fn incomplete_slide(pic: &Pic, pics: &Pics, scores_matrix: &ScoresMatrix) -> NextOption {
            fn gen_contender(idx: usize, pic: &Pic, contender: &Pic, axis_len: usize) -> Best {
                let tags = pic.intersect_with(contender);
                Best {
                    idx: idx,
                    pic: Pic {
                        tags: pic.union_with(contender),
                        id: axis_len,
                        source: PicSourceId::VV(pic.source(), contender.source()),
                    },
                    score: tags.len() as u8,
                    score_sum: 0,
                }
            }
            let axis_len = scores_matrix.len_of(Axis(0));
            let mut vpics: Vec<(usize, &Pic)> = vec![];
            for (idx, contender) in pics.iter().enumerate() {
                match contender.source {
                    PicSourceId::V(_) => vpics.push((idx, contender)),
                    _ => (),
                }
            }
            if vpics.is_empty() {
                return NextOption::None;
            }

            let best = gen_contender(vpics[0].0, &pic, vpics[0].1, axis_len);
            let mut contenders = vec![best.clone()];
            for (idx, contender) in vpics {
                let contender = gen_contender(idx, &pic, &contender, axis_len);
                if contender.score < best.score {
                    contenders.clear();
                    contenders.push(contender.clone());
                } else if contender.score == best.score {
                    contenders.push(contender.clone());
                }
            }
            let best = best_contender(contenders, axis_len, scores_matrix, pics);
            NextOption::Pic {
                pic: best.pic,
                other_idx: best.idx,
            }
        }
        match self.source {
            PicSourceId::H(_) => complete_slide(self, pics, scores_matrix),
            PicSourceId::V(_) => incomplete_slide(self, pics, scores_matrix),
            PicSourceId::VV(_, _) => complete_slide(self, pics, scores_matrix),
        }
    }
}

fn gen_score_sums(axis_len: usize, scores_matrix: &ScoresMatrix, pics: &Pics) -> ScoreSum {
    let mut score_sums_array = ScoreSum::zeros((axis_len,));
    for (ix, pic) in pics.iter().enumerate() {
        score_sums_array[[ix]] = pic.score_sum(axis_len, &scores_matrix, &pics)
    }
    score_sums_array
}

fn sort_pics(pics: &Pics, score_sums_array: &ScoreSum) -> Pics {
    let mut pics = pics.clone();
    pics.sort_by(|a, b| {
        score_sums_array[a.id]
            .cmp(&score_sums_array[b.id])
            .reverse()
    });
    pics.reindex();
    pics
}

pub fn iterative_greedy(pics: &Pics) -> Pics {
    let scores_matrix = pics.scores_matrix();
    let axis_len = scores_matrix.len_of(Axis(0));
    let score_sums_array = gen_score_sums(axis_len, &scores_matrix, &pics);
    let mut pics = sort_pics(&pics, &score_sums_array);

    let mut slides: Pics = vec![];
    let mut current_idx = 0usize;
    while !pics.is_empty() {
        let current = pics[current_idx].clone();
        slides.push(current.clone());
        pics.remove(current_idx);
        if pics.is_empty() {
            break;
        }
        current_idx = match current.next(&pics, &scores_matrix) {
            NextOption::Idx(idx) => idx,
            NextOption::Pic { pic, other_idx } => {
                slides.pop();
                let other = &pics[other_idx];
                pics.remove(other_idx);
                pics.push(pic);
                pics.len() - 1
            }
            NextOption::None => {
                slides.pop();
                match slides.pop() {
                    Some(pic) => {
                        pics.push(pic);
                        pics.len() - 1
                    }
                    None => 0,
                }
            }
        };
    }
    slides.reindex();
    slides
}

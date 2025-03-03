// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use martos::task_manager::{TaskManager, TaskManagerTrait};
// use std::collections::{LinkedList, VecDeque};
//
// const TASK_COUNT: usize = 100_000;
//
// fn bench_task_manager<Tasks>(c: &mut Criterion) {}
// criterion_group!(benches, bench_task_manager,);
// criterion_main!(benches);
// // fn bench_vec(c: &mut Criterion) {
// //     c.bench_function("vec push", |b| {
// //         b.iter(|| {
// //             let mut vec = Vec::new();
// //             for _ in 0..TASK_COUNT {
// //                 vec.push(black_box(Task));
// //             }
// //         })
// //     });
// //
// //     c.bench_function("vec remove front", |b| {
// //         b.iter(|| {
// //             let mut vec: Vec<_> = (0..TASK_COUNT).map(|_| Task).collect();
// //             for _ in 0..TASK_COUNT {
// //                 vec.remove(0);
// //             }
// //         })
// //     });
// //
// //     c.bench_function("vec remove middle", |b| {
// //         b.iter(|| {
// //             let mut vec: Vec<_> = (0..TASK_COUNT).map(|_| Task).collect();
// //             while vec.len() > 1 {
// //                 vec.remove(vec.len() / 2);
// //             }
// //         })
// //     });
// // }
// //
// // fn bench_vecdeque(c: &mut Criterion) {
// //     c.bench_function("vecdeque push", |b| {
// //         b.iter(|| {
// //             let mut deque = VecDeque::new();
// //             for _ in 0..TASK_COUNT {
// //                 deque.push_back(black_box(Task));
// //             }
// //         })
// //     });
// //
// //     c.bench_function("vecdeque pop front", |b| {
// //         b.iter(|| {
// //             let mut deque: VecDeque<_> = (0..TASK_COUNT).map(|_| Task).collect();
// //             for _ in 0..TASK_COUNT {
// //                 deque.pop_front();
// //             }
// //         })
// //     });
// //
// //     c.bench_function("vecdeque remove middle", |b| {
// //         b.iter(|| {
// //             let mut deque: VecDeque<_> = (0..TASK_COUNT).map(|_| Task).collect();
// //             while deque.len() > 1 {
// //                 let idx = deque.len() / 2;
// //                 deque.remove(idx);
// //             }
// //         })
// //     });
// // }
// //
// // fn bench_linkedlist(c: &mut Criterion) {
// //     c.bench_function("linkedlist push", |b| {
// //         b.iter(|| {
// //             let mut list = LinkedList::new();
// //             for _ in 0..TASK_COUNT {
// //                 list.push_back(black_box(Task));
// //             }
// //         })
// //     });
// //
// //     c.bench_function("linkedlist pop front", |b| {
// //         b.iter(|| {
// //             let mut list: LinkedList<_> = (0..TASK_COUNT).map(|_| Task).collect();
// //             for _ in 0..TASK_COUNT {
// //                 list.pop_front();
// //             }
// //         })
// //     });
// //
// //     c.bench_function("linkedlist remove middle", |b| {
// //         b.iter(|| {
// //             let mut list: LinkedList<_> = (0..TASK_COUNT).map(|_| Task).collect();
// //             while list.len() > 1 {
// //                 let mid = list.len() / 2;
// //                 let mut cursor = list.cursor_front_mut();
// //                 for _ in 0..mid {
// //                     cursor.move_next();
// //                 }
// //                 cursor.remove_current();
// //             }
// //         })
// //     });
// // }
// //
// // criterion_group!(benches, bench_vec, bench_vecdeque, bench_linkedlist);
// // criterion_main!(benches);

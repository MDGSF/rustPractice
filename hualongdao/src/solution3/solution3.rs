use crate::*;
use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt;

pub struct Solution3 {
  board: Board,                 // board 是个正方形
  fixed: usize,                 // 固定点的数字
  size: usize,                  // board 的边长
  stage: usize,                 // 第几关
  fixed_point: Point,           // fixed 的行列位置
  max_number: usize,            // 最大数字 size * size
  fixed_points: HashSet<Point>, // 当前不能被移动的点
  zero_point: Point,            // 空格的位置
  result: Vec<String>,          // 保存最后的结果，空格的移动命令，L R U D
  start_row: usize,
  start_col: usize,
  end_row: usize,
  end_col: usize,
}

impl Solution3 {
  pub fn new(input_context: &InputContext) -> Solution3 {
    let board = Board::new(input_context.board.clone());

    let fixed_point = board.number_to_point(input_context.fixed);

    let zero_point = board.find_num(0).unwrap();

    let max_number = input_context.size * input_context.size;

    let mut fixed_points: HashSet<Point> = HashSet::new();
    fixed_points.insert(fixed_point);

    Solution3 {
      board: board,
      fixed: input_context.fixed,
      size: input_context.size,
      stage: input_context.stage,
      fixed_point,
      max_number,
      fixed_points,
      zero_point,
      result: Vec::new(),
      start_row: 0,
      start_col: 0,
      end_row: input_context.size - 1,
      end_col: input_context.size - 1,
    }
  }

  pub fn process(&mut self) -> Result<()> {
    loop {
      if self.start_row >= self.end_row && self.start_col >= self.end_col {
        break;
      }

      if self.find_special_case() {
        continue;
      }

      if self.start_row <= self.start_col {
        self.process_start_row()?;
      } else {
        self.process_start_col()?;
      }

      info!("\n{}", self.board);

      println!();
    }

    Ok(())
  }

  fn process_start_row(&mut self) -> Result<()> {
    info!(
      "start process row, start_row = {}, start_col = {}",
      self.start_row, self.start_col,
    );
    // 处理 start_row 这一行

    // 一直从开始处理到倒数第 3 个数字
    for col in self.start_col..=(self.end_col - 2) {
      // num 是期望在点 [start_row, col] 这个位置上放置的数字
      let num = self.board.point_to_number(Point::new(self.start_row, col));
      // 把 num 数字移动到位置 [start_row, col] 这个位置上
      self.move_number(num)?;
    }

    self.process_row_last_2()?;

    self.start_row += 1;

    Ok(())
  }

  fn process_row_last_2(&mut self) -> Result<()> {
    if let Ok(_) = self.process_row_last_2_try_1() {
      return Ok(());
    }

    if let Ok(_) = self.process_row_last_2_try_2() {
      return Ok(());
    }

    if let Ok(_) = self.process_row_last_2_try_3() {
      return Ok(());
    }

    if let Ok(_) = self.process_row_last_2_try_4() {
      return Ok(());
    }

    if let Ok(_) = self.process_row_last_2_try_5() {
      return Ok(());
    }

    Err(anyhow!("process_row_last_2 failed"))
  }

  fn process_row_last_2_try_1(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_row 这一行的倒数第二个数字
    // last_1 是 start_row 这一行的最后一个数字
    // x x x p1 p2
    // x x x p3 p4
    let p1 = Point::new(self.start_row, self.end_col - 1);
    let p2 = Point::new(self.start_row, self.end_col);
    let _p3 = Point::new(self.start_row + 1, self.end_col - 1);
    let p4 = Point::new(self.start_row + 1, self.end_col);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_2, p2)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p4, vec![p2])?;
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  fn process_row_last_2_try_2(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_row 这一行的倒数第二个数字
    // last_1 是 start_row 这一行的最后一个数字
    // x x x p1 p2
    // x x x p3 p4
    //          p5
    let p1 = Point::new(self.start_row, self.end_col - 1);
    let p2 = Point::new(self.start_row, self.end_col);
    let _p3 = Point::new(self.start_row + 1, self.end_col - 1);
    let p4 = Point::new(self.start_row + 1, self.end_col);
    let p5 = Point::new(p4.row + 1, p4.col);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_2, p4)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p5, vec![p4])?;
      self.move_zero_to_dst_with_temp_fixed(p2, vec![p4, p5])?;
      self.move_zero_with_paths(vec![p4, p5]);
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  fn process_row_last_2_try_3(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_row 这一行的倒数第二个数字
    // last_1 是 start_row 这一行的最后一个数字
    // x x x p1 p2
    // x x x p3 p4
    //          p5
    //          p6
    let p1 = Point::new(self.start_row, self.end_col - 1);
    let p2 = Point::new(self.start_row, self.end_col);
    let _p3 = Point::new(self.start_row + 1, self.end_col - 1);
    let p4 = Point::new(self.start_row + 1, self.end_col);
    let p5 = Point::new(p4.row + 1, p4.col);
    let p6 = Point::new(p5.row + 1, p5.col);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_2, p5)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p6, vec![p5])?;
      self.move_zero_to_dst_with_temp_fixed(p4, vec![p5, p6])?;
      self.move_zero_with_paths(vec![p5, p6]);
      self.move_zero_to_dst_with_temp_fixed(p2, vec![p4, p5])?;
      self.move_zero_with_paths(vec![p4, p5]);
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  fn process_row_last_2_try_4(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_row 这一行的倒数第二个数字
    // last_1 是 start_row 这一行的最后一个数字
    // x x x p1 p2
    // x x x p3 p4
    //          p5
    //          p6
    //          p7
    let p1 = Point::new(self.start_row, self.end_col - 1);
    let p2 = Point::new(self.start_row, self.end_col);
    let _p3 = Point::new(self.start_row + 1, self.end_col - 1);
    let p4 = Point::new(self.start_row + 1, self.end_col);
    let p5 = Point::new(p4.row + 1, p4.col);
    let p6 = Point::new(p5.row + 1, p5.col);
    let p7 = Point::new(p6.row + 1, p6.col);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_2, p6)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p7, vec![p6])?;

      self.move_zero_to_dst_with_temp_fixed(p5, vec![p6, p7])?;
      self.move_zero_with_paths(vec![p6, p7]);
      self.move_zero_to_dst_with_temp_fixed(p4, vec![p5, p6])?;
      self.move_zero_with_paths(vec![p5, p6]);
      self.move_zero_to_dst_with_temp_fixed(p2, vec![p4, p5])?;
      self.move_zero_with_paths(vec![p4, p5]);
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  fn process_row_last_2_try_5(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_row 这一行的倒数第二个数字
    // last_1 是 start_row 这一行的最后一个数字
    // x x x p1 p2
    // x x x p3 p4
    //          p5
    // .....
    // p6 p7
    let p1 = Point::new(self.start_row, self.end_col - 1);
    let p2 = Point::new(self.start_row, self.end_col);
    let _p3 = Point::new(self.start_row + 1, self.end_col - 1);
    let p4 = Point::new(self.start_row + 1, self.end_col);
    let p5 = Point::new(p4.row + 1, p4.col);

    let p6 = Point::new(self.end_row, self.start_col);
    let p7 = Point::new(self.end_row, self.start_col + 1);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_1, p6)?;
      self.move_number_to_dst(last_2, p7)?;

      self.move_number_to_dst(last_2, p4)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p5, vec![p4])?;

      self.move_zero_to_dst_with_temp_fixed(p2, vec![p4, p5])?;
      self.move_zero_with_paths(vec![p4, p5]);
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  fn process_start_col(&mut self) -> Result<()> {
    info!(
      "start process col, start_row = {}, start_col = {}",
      self.start_row, self.start_col,
    );
    // 处理 start_col 这一列

    for row in self.start_row..=(self.end_row - 2) {
      // num 是期望在点 [start_row, col] 这个位置上放置的数字
      let num = self.board.point_to_number(Point::new(row, self.start_col));
      // 把 num 数字移动到位置 [row, start_col] 这个位置上
      self.move_number(num)?;
    }

    self.process_col_last_2()?;

    self.start_col += 1;
    Ok(())
  }

  fn process_col_last_2(&mut self) -> Result<()> {
    if let Ok(_) = self.process_col_last_2_try_1() {
      return Ok(());
    }

    if let Ok(_) = self.process_col_last_2_try_2() {
      return Ok(());
    }

    Err(anyhow!("process_col_last_2 failed"))
  }

  fn process_col_last_2_try_1(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_col 这一列的倒数第二个数字
    // last_1 是 start_col 这一列的最后一个数字
    // x  x
    // x  x
    // x  x
    // p1 p3
    // p2 p4 p5 p6

    let p1 = Point::new(self.end_row - 1, self.start_col);
    let p2 = Point::new(self.end_row, self.start_col);
    let _p3 = Point::new(self.end_row - 1, self.start_col + 1);
    let p4 = Point::new(self.end_row, self.start_col + 1);
    let p5 = Point::new(p4.row, p4.col + 1);
    let p6 = Point::new(p5.row, p5.col + 1);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_2, p5)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p6, vec![p5])?;
      self.move_zero_to_dst_with_temp_fixed(p4, vec![p5, p6])?;
      self.move_zero_with_paths(vec![p5, p6]);
      self.move_zero_to_dst_with_temp_fixed(p2, vec![p4, p5])?;
      self.move_zero_with_paths(vec![p4, p5]);
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  fn process_col_last_2_try_2(&mut self) -> Result<()> {
    // 最后两个数字特殊处理
    // last_2 是 start_col 这一列的倒数第二个数字
    // last_1 是 start_col 这一列的最后一个数字
    // x  x
    // x  x
    // x  x
    // p1 p3
    // p2 p4 p5 p6

    let p1 = Point::new(self.end_row - 1, self.start_col);
    let p2 = Point::new(self.end_row, self.start_col);
    let _p3 = Point::new(self.end_row - 1, self.start_col + 1);
    let p4 = Point::new(self.end_row, self.start_col + 1);

    let last_2 = self.board.point_to_number(p1);
    let last_1 = self.board.point_to_number(p2);

    if self.board[p1] == last_2 && self.board[p2] == last_1 {
      // 最后两个数字已经是正确的了，就不需要处理了
    } else {
      self.move_number_to_dst(last_2, p2)?;
      self.move_number_to_dst_with_temp_fixed(last_1, p4, vec![p2])?;
      self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p4])?;
      self.move_zero_with_paths(vec![p2, p4]);
    }

    self.fixed_points.insert(p1);
    self.fixed_points.insert(p2);

    Ok(())
  }

  pub(crate) fn move_number(&mut self, num: usize) -> Result<()> {
    info!("move start num = {}", num);

    let num_point = self.board.number_to_point(num);

    if num % self.size == 0 {
      self.move_row_last_number(num)?;
    } else if num_point.row == self.size - 1 {
      self.move_col_last_number(num)?;
    } else {
      self.move_basic_number(num)?;
    }

    self.fixed_points.insert(num_point);

    Ok(())
  }

  fn move_row_last_number(&mut self, num: usize) -> Result<()> {
    let src_point = self.board.find_num(num).unwrap();
    let dst_point = self.board.number_to_point(num);
    if src_point == dst_point {
      return Ok(());
    }
    let mut pre_dst_point = dst_point;
    pre_dst_point.col += 1;
    self.move_number_from_src_to_dst(num, src_point, pre_dst_point)?;

    // TODO
    if self.zero_point == dst_point {
      self.swap_with_zero(pre_dst_point);
    }

    Ok(())
  }

  fn move_col_last_number(&mut self, num: usize) -> Result<()> {
    let src_point = self.board.find_num(num).unwrap();
    let dst_point = self.board.number_to_point(num);
    if src_point == dst_point {
      return Ok(());
    }
    let mut pre_dst_point = dst_point;
    pre_dst_point.col += 1;
    self.move_number_from_src_to_dst(num, src_point, pre_dst_point)?;

    // TODO
    if self.zero_point == dst_point {
      self.swap_with_zero(pre_dst_point);
    }

    Ok(())
  }

  pub(crate) fn move_basic_number(&mut self, num: usize) -> Result<()> {
    let src_point = self.board.find_num(num).unwrap();
    let dst_point = self.board.number_to_point(num);
    if src_point == dst_point {
      return Ok(());
    }
    self.move_number_from_src_to_dst(num, src_point, dst_point)?;
    Ok(())
  }

  // 把数字 num 移动到 dst_point 的位置
  // 并在移动的过程中把 temp_fixed 数组中的 point 设置为固定点
  pub(crate) fn move_number_to_dst_with_temp_fixed(
    &mut self,
    num: usize,
    dst_point: Point,
    temp_fixed: Vec<Point>,
  ) -> Result<()> {
    for &point in temp_fixed.iter() {
      self.fixed_points.insert(point);
    }

    let result = self.move_number_to_dst(num, dst_point);

    for point in temp_fixed.iter() {
      self.fixed_points.remove(&point);
    }

    result
  }

  pub(crate) fn move_number_to_dst(&mut self, num: usize, dst_point: Point) -> Result<()> {
    let src_point = self.board.find_num(num).unwrap();
    self.move_number_from_src_to_dst(num, src_point, dst_point)?;
    Ok(())
  }

  pub(crate) fn move_number_from_src_to_dst(
    &mut self,
    num: usize,
    src_point: Point,
    dst_point: Point,
  ) -> Result<()> {
    if src_point == dst_point {
      return Ok(());
    } else {
      info!("move {} from {:?} to {:?}", num, src_point, dst_point);
    }

    let num_paths = self.find_path(num, src_point, dst_point);
    if num_paths.is_none() {
      info!("move_number_from_src_to_dst find special case: {}", self);
      return Err(anyhow!("move_number_from_src_to_dst find special case"));
    }
    let num_paths = num_paths.unwrap();
    // info!("num_paths = {:?}", num_paths);

    let mut num_point = src_point;
    for path_point in num_paths {
      // 先把 0 移动到要移动的数字前面
      self.move_zero_to_dst_with_temp_fixed(path_point, vec![num_point])?;

      // 把数字向前移动一步
      self.swap_with_zero(num_point);
      num_point = path_point;
    }

    Ok(())
  }

  // 把 self.zero_point 移动到 dst_point 的位置
  // 并在移动的过程中把 temp_fixed 数组中的 point 设置为固定点
  pub(crate) fn move_zero_to_dst_with_temp_fixed(
    &mut self,
    dst_point: Point,
    temp_fixed: Vec<Point>,
  ) -> Result<()> {
    if self.zero_point == dst_point {
      return Ok(());
    }

    for &point in temp_fixed.iter() {
      self.fixed_points.insert(point);
    }

    let result = self.move_zero_to_dst(dst_point);

    for point in temp_fixed.iter() {
      self.fixed_points.remove(&point);
    }

    result
  }

  // 把 self.zero_point 移动到 dst_point 的位置
  pub(crate) fn move_zero_to_dst(&mut self, dst_point: Point) -> Result<()> {
    if self.zero_point == dst_point {
      return Ok(());
    }

    let zero_paths = self.find_path(0, self.zero_point, dst_point);
    if zero_paths.is_none() {
      info!("{}", self);
      return Err(anyhow!("find special case, dst_point = {:?}", dst_point));
    }
    let zero_paths = zero_paths.unwrap();

    self.move_zero_with_paths(zero_paths);

    Ok(())
  }

  // 让 self.zero_point 沿着 zero_paths 移动
  pub(crate) fn move_zero_with_paths(&mut self, zero_paths: Vec<Point>) {
    for &path_point in zero_paths.iter() {
      self.swap_with_zero(path_point);
    }
  }

  // 查找从 src_point 到 dst_point 的移动路径
  // 返回的移动路径，不包含 src_point, 包含 dst_point
  // 移动时，无法跨越固定点
  pub(crate) fn find_path(
    &mut self,
    _num: usize,
    src_point: Point,
    dst_point: Point,
  ) -> Option<Vec<Point>> {
    let context = BFSContext {
      position: src_point,
      manhattan_distance: calc_two_point_manhattan_distance(src_point, dst_point),
      path: vec![],
    };

    let mut s = HashSet::new();
    s.insert(src_point);

    let mut heap = BinaryHeap::new();
    heap.push(context);

    while !heap.is_empty() {
      let context = heap.pop().unwrap();
      let point = context.position;

      for direction in DIRECTIONS.iter() {
        let new_ipoint = point + direction;
        if !self.is_valid_ipoint(&new_ipoint) {
          continue;
        }

        let new_upoint = Point::from(new_ipoint);

        if new_upoint == dst_point {
          let mut new_path = context.path.clone();
          new_path.push(new_upoint);
          return Some(new_path);
        }

        let t1 = s.contains(&new_upoint);
        let t2 = self.is_fixed_upoint(&new_upoint);

        if !t1 && !t2 {
          s.insert(new_upoint);
          let mut new_path = context.path.clone();
          new_path.push(new_upoint);
          let new_context = BFSContext {
            position: new_upoint,
            manhattan_distance: calc_two_point_manhattan_distance(new_upoint, dst_point),
            path: new_path,
          };
          heap.push(new_context);
        }
      }
    }

    None
  }

  // 判断 point 点是否在正方形内
  pub(crate) fn is_valid_ipoint(&self, point: &IPoint) -> bool {
    let size = self.size as isize;
    point.row >= 0 && point.row < size && point.col >= 0 && point.col < size
  }

  // 判断 point 是否是固定点
  pub(crate) fn is_fixed_upoint(&self, point: &Point) -> bool {
    self.fixed_points.contains(point)
  }

  // 1. 交换 zero_point 和 point 的值
  // 2. 并更新 self.zero_point 的位置
  // 3. 记录 self.zero_point 移动的路径
  pub(crate) fn swap_with_zero(&mut self, point: Point) {
    self.record_zero_point_move_poing(point);
    self.board.swap_points(self.zero_point, point);
    self.zero_point = point;
  }

  // 记录 self.zero_point 移动的路径
  pub(crate) fn record_zero_point_move_poing(&mut self, point: Point) {
    for direction in DIRECTIONS.iter() {
      let ipoint = self.zero_point + direction;
      let upoint = Point::from(ipoint);
      if upoint == point {
        self.result.push(direction.name.to_string());
        return;
      }
    }
    panic!(
      "swap invalid, zero_point = {:?}, point = {:?}",
      self.zero_point, point
    );
  }

  fn find_special_case(&mut self) -> bool {
    if self.special_1_condition() {
      match self.specail_1_try_1() {
        Ok(_) => return true,
        Err(err) => {
          error!("{}", err);
        }
      }

      match self.specail_1_try_2() {
        Ok(_) => return true,
        Err(err) => {
          error!("{}", err);
        }
      }

      match self.specail_1_try_3() {
        Ok(_) => return true,
        Err(err) => {
          error!("{}", err);
        }
      }

      panic!("specail case 1 failed");

      return false;
    }

    if self.special_2_condition() {
      self.specail_2_process();
      return true;
    }

    false
  }

  fn special_1_condition(&self) -> bool {
    let p1_num = self.fixed - 1;
    let p3_num = self.fixed - self.size;
    let p2_num = p3_num - 1;

    let frow = self.fixed_point.row;
    let fcol = self.fixed_point.col;
    let p1 = Point::new(frow, fcol - 1);
    let p2 = Point::new(frow - 1, fcol - 1);
    let p3 = Point::new(frow - 1, fcol);

    if self.fixed_point.row == self.start_row + 1 && self.fixed_point.col == self.start_col + 1 {
      if self.board[p1] == p1_num && self.board[p2] == p2_num && self.board[p3] == p3_num {
        return false;
      } else {
        return true;
      }
    }
    false
  }

  // x x  x  x  x
  // x p2 p3 p4
  // x p1 f  p5
  // x x  x  p6
  // x x  x  p7
  fn specail_1_try_1(&mut self) -> Result<()> {
    info!("specail_1_try_1 start");
    let p1_num = self.fixed - 1;
    let p3_num = self.fixed - self.size;
    let p2_num = p3_num - 1;

    let frow = self.fixed_point.row;
    let fcol = self.fixed_point.col;
    let p1 = Point::new(frow, fcol - 1);
    let p2 = Point::new(frow - 1, fcol - 1);
    let p3 = Point::new(frow - 1, fcol);
    let p4 = Point::new(frow - 1, fcol + 1);
    let p5 = Point::new(frow, fcol + 1);
    let p6 = Point::new(frow + 1, fcol + 1);

    self.move_number_to_dst(p1_num, p4)?;
    self.move_number_to_dst_with_temp_fixed(p2_num, p5, vec![p4])?;
    self.move_number_to_dst_with_temp_fixed(p3_num, p6, vec![p4, p5])?;
    self.move_zero_to_dst_with_temp_fixed(p3, vec![p4, p5, p6])?;
    self.move_zero_with_paths(vec![p4, p5, p6]);
    self.move_zero_to_dst_with_temp_fixed(p2, vec![p3, p4, p5])?;
    self.move_zero_with_paths(vec![p3, p4, p5]);
    self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p3, p4])?;
    self.move_zero_with_paths(vec![p2, p3, p4]);

    Ok(())
  }

  // x x  x  x  x
  // x p2 p3 p4
  // x p1 f  p5
  // x x  x  p6
  // x x  x  p7
  fn specail_1_try_2(&mut self) -> Result<()> {
    info!("specail_1_try_2 start");
    let p1_num = self.fixed - 1;
    let p3_num = self.fixed - self.size;
    let p2_num = p3_num - 1;

    let frow = self.fixed_point.row;
    let fcol = self.fixed_point.col;
    let p1 = Point::new(frow, fcol - 1);
    let p2 = Point::new(frow - 1, fcol - 1);
    let p3 = Point::new(frow - 1, fcol);
    let p4 = Point::new(frow - 1, fcol + 1);
    let p5 = Point::new(frow, fcol + 1);
    let p6 = Point::new(frow + 1, fcol + 1);

    self.move_number_to_dst(p1_num, p3)?;
    self.move_number_to_dst_with_temp_fixed(p2_num, p4, vec![p3])?;
    self.move_number_to_dst_with_temp_fixed(p3_num, p5, vec![p3, p4])?;
    self.move_zero_to_dst_with_temp_fixed(p2, vec![p3, p4, p5])?;
    self.move_zero_with_paths(vec![p3, p4, p5]);
    self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p3, p4])?;
    self.move_zero_with_paths(vec![p2, p3, p4]);

    Ok(())
  }

  // x x  x  x  x
  // x p2 p3 p4
  // x p1 f  p5
  // x x  x  p6
  // x x  x  x   x
  // x x  .....  p7
  //      x  p8  p9
  fn specail_1_try_3(&mut self) -> Result<()> {
    info!("specail_1_try_3 start");
    let p1_num = self.fixed - 1;
    let p3_num = self.fixed - self.size;
    let p2_num = p3_num - 1;

    let frow = self.fixed_point.row;
    let fcol = self.fixed_point.col;
    let p1 = Point::new(frow, fcol - 1);
    let p2 = Point::new(frow - 1, fcol - 1);
    let p3 = Point::new(frow - 1, fcol);
    let p4 = Point::new(frow - 1, fcol + 1);
    let p5 = Point::new(frow, fcol + 1);

    let p7 = Point::new(self.end_row - 1, self.end_col);
    let p8 = Point::new(self.end_row, self.end_col - 1);
    let p9 = Point::new(self.end_row, self.end_col);

    self.move_number_to_dst(p3_num, p9)?;
    self.move_number_to_dst(p2_num, p8)?;
    self.move_number_to_dst(p1_num, p7)?;

    self.move_number_to_dst(p1_num, p3)?;
    self.move_number_to_dst_with_temp_fixed(p2_num, p4, vec![p3])?;
    self.move_number_to_dst_with_temp_fixed(p3_num, p5, vec![p3, p4])?;
    self.move_zero_to_dst_with_temp_fixed(p2, vec![p3, p4, p5])?;
    self.move_zero_with_paths(vec![p3, p4, p5]);
    self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p3, p4])?;
    self.move_zero_with_paths(vec![p2, p3, p4]);

    Ok(())
  }

  // 只剩下 3 * 3 大小的空间了，改用暴力搜索
  fn special_2_condition(&self) -> bool {
    let left_width = self.end_col - self.start_col + 1;
    let left_height = self.end_row - self.start_row + 1;
    if left_width == 3 && left_height == 3 {
      return true;
    }
    false
  }

  fn specail_2_process(&mut self) {
    info!("specail_2_process: {}", self);

    let input_ctx = self.generate_input_context();
    let mut solution1 = Solution1::new(&input_ctx);
    let result = solution1.search_bfs().unwrap();
    self.result.push(result);

    self.start_row = self.end_row;
    self.start_col = self.end_col;
  }

  fn generate_input_context(&self) -> InputContext {
    let size = 3;
    let mut board = vec![vec![0_usize; size]; size];
    let mut fixed: usize = 0;
    let mut size: usize = 0;
    let stage: usize = self.stage;

    let left_top = Point::new(self.start_row, self.start_col);
    let left_top_num = self.board.point_to_number(left_top);
    let right_bottom = Point::new(self.end_col, self.end_col);
    let rect = Rect::new(left_top, right_bottom);

    if rect.contains(self.fixed_point) {
      let fixed_point = calc_two_point_relative_position(left_top, self.fixed_point);
      fixed = point_to_number(size, fixed_point);
    }

    let left_width = self.end_col - self.start_col + 1;
    let left_height = self.end_row - self.start_row + 1;
    assert!(left_width == left_height);

    size = left_width;

    let mut new_row = 0;
    for row in self.start_row..=self.end_row {
      let mut new_col = 0;
      for col in self.start_col..=self.end_col {
        let num = self.board.num(Point::new(row, col));
        if num == 0 {
          board[new_row][new_col] = 0;
        } else {
          let num_point = self.board.number_to_point(num);
          let new_num_point = calc_two_point_relative_position(left_top, num_point);
          let new_num = new_num_point.row * size + new_num_point.col + 1;
          board[new_row][new_col] = new_num;
        }
        new_col += 1;
      }
      new_row += 1;
    }

    InputContext {
      board,
      fixed,
      size,
      stage,
    }
  }
}

impl fmt::Display for Solution3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut result = String::new();
    result.push_str(&format!("stage: {}\n", self.stage));
    result.push_str(&format!(
      "size: {} * {} = {}\n",
      self.size, self.size, self.max_number,
    ));

    result.push_str(&format!("{}", self.board));

    result.push_str(&format!("fixed: {}, {:?}\n", self.fixed, self.fixed_point));

    let mut fixed_points: Vec<_> = self.fixed_points.iter().collect();
    fixed_points.sort();
    result.push_str(&format!("fixed_points: {:?}\n", fixed_points));
    result.push_str(&format!("zero_point: {:?}\n", self.zero_point));
    result.push_str(&format!(
      "start_row: {}, start_col: {}\n",
      self.start_row, self.start_col
    ));
    result.push_str(&format!(
      "end_row: {}, end_col: {}\n",
      self.end_row, self.end_col
    ));
    result.push_str(&format!("result: {}\n", self.result.join("")));
    write!(f, "{}", result)
  }
}

#[derive(Debug, Clone, Eq)]
struct BFSContext {
  position: Point,
  manhattan_distance: usize,
  path: Vec<Point>,
}

impl Ord for BFSContext {
  fn cmp(&self, other: &Self) -> Ordering {
    other.manhattan_distance.cmp(&self.manhattan_distance)
  }
}

impl PartialOrd for BFSContext {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for BFSContext {
  fn eq(&self, other: &Self) -> bool {
    self.manhattan_distance == other.manhattan_distance
  }
}

#[derive(Debug, Clone, Eq)]
struct AStarContext {
  manhattan_distance: usize,
  board_str: String,
  path: Vec<String>,
}

impl Ord for AStarContext {
  fn cmp(&self, other: &Self) -> Ordering {
    other.manhattan_distance.cmp(&self.manhattan_distance)
  }
}

impl PartialOrd for AStarContext {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for AStarContext {
  fn eq(&self, other: &Self) -> bool {
    self.manhattan_distance == other.manhattan_distance
  }
}

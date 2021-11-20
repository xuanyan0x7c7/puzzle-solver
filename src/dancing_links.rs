struct Node {
    row: usize,
    column: usize,
}

struct Row {
    head: usize,
    choose: bool,
}

struct Column {
    head: usize,
    count: usize,
}

struct State {
    column: usize,
    row_node: usize,
}

pub struct DancingLinks {
    node_list: Vec<Node>,
    row_list: Vec<Row>,
    column_list: Vec<Column>,
    up: Vec<usize>,
    down: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
    state_stack: Vec<State>,
}

impl DancingLinks {
    pub fn new() -> DancingLinks {
        let mut board = DancingLinks {
            node_list: vec![],
            row_list: vec![],
            column_list: vec![],
            up: vec![],
            down: vec![],
            left: vec![],
            right: vec![],
            state_stack: vec![State {
                column: usize::MAX,
                row_node: usize::MAX,
            }],
        };
        board.new_node(usize::MAX, usize::MAX, usize::MAX, usize::MAX);
        board
    }

    fn new_node(
        &mut self,
        row: usize,
        column: usize,
        row_head: usize,
        column_head: usize,
    ) -> usize {
        let node_index = self.node_list.len();
        self.node_list.push(Node { row, column });
        self.up.push(usize::MAX);
        self.down.push(usize::MAX);
        self.left.push(usize::MAX);
        self.right.push(usize::MAX);
        if row_head == usize::MAX {
            self.left[node_index] = node_index;
            self.right[node_index] = node_index;
        } else {
            self.left[node_index] = self.left[row_head];
            self.right[node_index] = row_head;
            self.right[self.left[node_index]] = node_index;
            self.left[row_head] = node_index;
        }
        if column_head == usize::MAX {
            self.up[node_index] = node_index;
            self.down[node_index] = node_index;
        } else {
            self.up[node_index] = self.up[column_head];
            self.down[node_index] = column_head;
            self.down[self.up[node_index]] = node_index;
            self.up[column_head] = node_index;
        }
        node_index
    }

    pub fn add_rows(&mut self, row_count: usize) {
        let column_index = self.column_list.len();
        let column = Column {
            head: self.new_node(usize::MAX, column_index, 0, usize::MAX),
            count: row_count,
        };
        for _ in 0..row_count {
            let row = Row {
                head: self.new_node(self.row_list.len(), column_index, usize::MAX, column.head),
                choose: false,
            };
            self.row_list.push(row);
        }
        self.column_list.push(column);
    }

    pub fn add_constraint(&mut self, rows: &Vec<usize>, unique: bool) {
        let column_index = self.column_list.len();
        let column = Column {
            head: self.new_node(
                usize::MAX,
                column_index,
                if unique { 0 } else { usize::MAX },
                usize::MAX,
            ),
            count: rows.len(),
        };
        for row in rows {
            self.new_node(*row, column_index, self.row_list[*row].head, column.head);
        }
        self.column_list.push(column);
    }

    fn remove(&mut self, column: usize) {
        let column_head = self.column_list[column].head;
        self.right[self.left[column_head]] = self.right[column_head];
        self.left[self.right[column_head]] = self.left[column_head];
        let mut row_node = self.down[column_head];
        while row_node != column_head {
            let mut column_node = self.right[row_node];
            while column_node != row_node {
                self.column_list[self.node_list[column_node].column].count -= 1;
                self.down[self.up[column_node]] = self.down[column_node];
                self.up[self.down[column_node]] = self.up[column_node];
                column_node = self.right[column_node];
            }
            row_node = self.down[row_node];
        }
    }

    fn resume(&mut self, column: usize) {
        let column_head = self.column_list[column].head;
        let mut row_node = self.up[column_head];
        while row_node != column_head {
            let mut column_node = self.left[row_node];
            while column_node != row_node {
                self.column_list[self.node_list[column_node].column].count += 1;
                self.down[self.up[column_node]] = column_node;
                self.up[self.down[column_node]] = column_node;
                column_node = self.left[column_node];
            }
            row_node = self.up[row_node];
        }
        self.right[self.left[column_head]] = column_head;
        self.left[self.right[column_head]] = column_head;
    }

    pub fn solve(&mut self) -> SolverIterator {
        SolverIterator { solver: self }
    }

    pub(crate) fn solve_next(&mut self) -> Option<Vec<usize>> {
        loop {
            let stack_top = self.state_stack.pop();
            if stack_top.is_none() {
                return None;
            }
            let state = stack_top.unwrap();
            if state.column != usize::MAX {
                let mut column_node = self.left[state.row_node];
                while column_node != state.row_node {
                    self.resume(self.node_list[column_node].column);
                    column_node = self.left[column_node];
                }
                self.row_list[self.node_list[state.row_node].row].choose = false;
                let row_node = self.down[state.row_node];
                if row_node == self.column_list[state.column].head {
                    self.resume(state.column);
                    continue;
                } else {
                    self.row_list[self.node_list[row_node].row].choose = true;
                    let mut column_node = self.right[row_node];
                    while column_node != row_node {
                        self.remove(self.node_list[column_node].column);
                        column_node = self.right[column_node];
                    }
                    self.state_stack.push(State {
                        column: state.column,
                        row_node,
                    });
                }
            }
            if self.right[0] == 0 {
                return Some(
                    self.row_list
                        .iter()
                        .enumerate()
                        .filter(|(_, row)| row.choose)
                        .map(|(index, _)| index)
                        .collect(),
                );
            }
            let min_column = self.pick_best_column();
            if min_column.is_none() {
                continue;
            }
            let best_column = min_column.unwrap();
            self.remove(best_column);
            let column_head = self.column_list[best_column].head;
            let row_node = self.down[column_head];
            if row_node != column_head {
                self.row_list[self.node_list[row_node].row].choose = true;
                let mut column_node = self.right[row_node];
                while column_node != row_node {
                    self.remove(self.node_list[column_node].column);
                    column_node = self.right[column_node];
                }
                self.state_stack.push(State {
                    column: best_column,
                    row_node,
                });
                self.state_stack.push(State {
                    column: usize::MAX,
                    row_node: usize::MAX,
                });
            }
        }
    }

    fn pick_best_column(&self) -> Option<usize> {
        let mut min = (usize::MAX, usize::MAX);
        let mut column_head = self.right[0];
        while column_head != 0 {
            let column = self.node_list[column_head].column;
            if self.column_list[column].count < min.0 {
                min.0 = self.column_list[column].count;
                min.1 = column;
                if min.0 == 1 {
                    return Some(min.1);
                } else if min.0 == 0 {
                    return None;
                }
            }
            column_head = self.right[column_head]
        }
        Some(min.1)
    }
}

pub struct SolverIterator<'a> {
    solver: &'a mut DancingLinks,
}

impl<'a> Iterator for SolverIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.solve_next()
    }
}

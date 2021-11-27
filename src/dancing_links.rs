struct Node {
    row: usize,
    column: usize,
}

struct Row {
    head: usize,
    chosen: bool,
}

enum ColumnType {
    Unique,
    CombinationalUnique,
    Constraint,
}

struct Column {
    column_type: ColumnType,
    head: usize,
    count: usize,
}

impl Column {
    fn is_combinational_unique(&self) -> bool {
        match self.column_type {
            ColumnType::CombinationalUnique => true,
            _ => false,
        }
    }
}

struct State {
    column: usize,
    row_node: usize,
    trigger_chaining: bool,
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
    holes: usize,
    current_holes: usize,
    chaining: bool,
}

impl DancingLinks {
    pub fn new() -> Self {
        Self::new_with_holes(0)
    }

    pub fn new_with_holes(holes: usize) -> Self {
        let mut board = Self {
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
                trigger_chaining: false,
            }],
            holes,
            current_holes: 0,
            chaining: false,
        };
        board.new_head_node();
        board.new_head_node();
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

    fn new_head_node(&mut self) -> usize {
        self.new_node(usize::MAX, usize::MAX, usize::MAX, usize::MAX)
    }

    fn new_column(&mut self, column_type: ColumnType, count: usize) -> Column {
        let column_index = self.column_list.len();
        let head = self.new_node(
            usize::MAX,
            column_index,
            match column_type {
                ColumnType::Unique => 0,
                ColumnType::CombinationalUnique => 1,
                ColumnType::Constraint => usize::MAX,
            },
            usize::MAX,
        );
        Column {
            column_type,
            head,
            count,
        }
    }

    pub fn add_rows(&mut self, row_count: usize) {
        let column_index = self.column_list.len();
        let column = self.new_column(ColumnType::Unique, row_count);
        for _ in 0..row_count {
            let row = Row {
                head: self.new_node(self.row_list.len(), column_index, usize::MAX, column.head),
                chosen: false,
            };
            self.row_list.push(row);
        }
        self.column_list.push(column);
    }

    pub fn add_column(&mut self, rows: &Vec<usize>, unique: bool) {
        let column_index = self.column_list.len();
        let column = self.new_column(
            if unique {
                ColumnType::Unique
            } else {
                ColumnType::CombinationalUnique
            },
            rows.len(),
        );
        for row in rows {
            self.new_node(*row, column_index, self.row_list[*row].head, column.head);
        }
        self.column_list.push(column);
    }

    pub fn add_constraint(&mut self, rows: &Vec<usize>) {
        let column_index = self.column_list.len();
        let column = self.new_column(ColumnType::Constraint, rows.len());
        for row in rows {
            self.new_node(*row, column_index, self.row_list[*row].head, column.head);
        }
        self.column_list.push(column);
    }

    pub fn select_row(&mut self, row: usize) {
        let row_head = self.row_list[row].head;
        let mut row_nodes = vec![row_head];
        let mut node = self.right[row_head];
        while node != row_head {
            row_nodes.push(node);
            node = self.right[node];
        }
        for node in row_nodes {
            self.remove_column(self.node_list[node].column);
        }
        self.row_list[row].chosen = true;
    }

    pub fn deselect_row(&mut self, row: usize) {
        let row_head = self.row_list[row].head;
        let mut node = row_head;
        loop {
            if self.down[self.up[node]] == node {
                let column_item = &mut self.column_list[self.node_list[node].column];
                column_item.count -= 1;
                if column_item.is_combinational_unique() && column_item.count == 0 {
                    self.current_holes += 1;
                }
                self.down[self.up[node]] = self.down[node];
                self.up[self.down[node]] = self.up[node];
            }
            node = self.right[node];
            if node == row_head {
                break;
            }
        }
    }

    fn remove_column(&mut self, column: usize) {
        let column_item = &mut self.column_list[column];
        if column_item.is_combinational_unique() && column_item.count == 0 {
            self.current_holes -= 1;
        }
        let column_head = column_item.head;
        self.right[self.left[column_head]] = self.right[column_head];
        self.left[self.right[column_head]] = self.left[column_head];
        let mut row_node = self.down[column_head];
        while row_node != column_head {
            let mut column_node = self.right[row_node];
            while column_node != row_node {
                let column_item = &mut self.column_list[self.node_list[column_node].column];
                column_item.count -= 1;
                if column_item.is_combinational_unique() && column_item.count == 0 {
                    self.current_holes += 1;
                }
                self.down[self.up[column_node]] = self.down[column_node];
                self.up[self.down[column_node]] = self.up[column_node];
                column_node = self.right[column_node];
            }
            row_node = self.down[row_node];
        }
    }

    fn resume_column(&mut self, column: usize) {
        let column_item = &mut self.column_list[column];
        if column_item.is_combinational_unique() && column_item.count == 0 {
            self.current_holes += 1;
        }
        let column_head = column_item.head;
        self.right[self.left[column_head]] = column_head;
        self.left[self.right[column_head]] = column_head;
        let mut row_node = self.up[column_head];
        while row_node != column_head {
            let mut column_node = self.left[row_node];
            while column_node != row_node {
                let column_item = &mut self.column_list[self.node_list[column_node].column];
                column_item.count += 1;
                if column_item.is_combinational_unique() && column_item.count == 1 {
                    self.current_holes -= 1;
                }
                self.down[self.up[column_node]] = column_node;
                self.up[self.down[column_node]] = column_node;
                column_node = self.left[column_node];
            }
            row_node = self.up[row_node];
        }
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
                    self.resume_column(self.node_list[column_node].column);
                    column_node = self.left[column_node];
                }
                self.row_list[self.node_list[state.row_node].row].chosen = false;
                let row_node = self.down[state.row_node];
                if row_node == self.column_list[state.column].head {
                    self.resume_column(state.column);
                    if state.trigger_chaining {
                        self.chaining = false;
                    }
                    continue;
                } else {
                    self.row_list[self.node_list[row_node].row].chosen = true;
                    let mut column_node = self.right[row_node];
                    while column_node != row_node {
                        self.remove_column(self.node_list[column_node].column);
                        column_node = self.right[column_node];
                    }
                    self.state_stack.push(State {
                        column: state.column,
                        row_node,
                        trigger_chaining: state.trigger_chaining,
                    });
                }
            }
            if self.right[0] == 0 {
                return Some(
                    self.row_list
                        .iter()
                        .enumerate()
                        .filter(|(_, row)| row.chosen)
                        .map(|(index, _)| index)
                        .collect(),
                );
            }
            if self.holes > 0 && self.current_holes > self.holes {
                continue;
            }
            let trigger_chaining =
                !self.chaining && self.holes > 0 && self.current_holes == self.holes;
            if trigger_chaining {
                self.chaining = true;
            }
            let min_column = self.pick_best_column();
            if min_column.is_none() {
                if trigger_chaining {
                    self.chaining = false;
                }
                continue;
            }
            let best_column = min_column.unwrap();
            self.remove_column(best_column);
            let column_head = self.column_list[best_column].head;
            let row_node = self.down[column_head];
            if row_node != column_head {
                self.row_list[self.node_list[row_node].row].chosen = true;
                let mut column_node = self.right[row_node];
                while column_node != row_node {
                    self.remove_column(self.node_list[column_node].column);
                    column_node = self.right[column_node];
                }
                self.state_stack.push(State {
                    column: best_column,
                    row_node,
                    trigger_chaining,
                });
                self.state_stack.push(State {
                    column: usize::MAX,
                    row_node: usize::MAX,
                    trigger_chaining: false,
                });
            }
        }
    }

    fn pick_best_column(&self) -> Option<usize> {
        let mut min = (usize::MAX, usize::MAX);
        let mut column_head = self.right[0];
        while column_head != 0 {
            let column = self.node_list[column_head].column;
            let count = self.column_list[column].count;
            if count < min.0 {
                if count == 1 {
                    return Some(column);
                } else if count == 0 {
                    return None;
                }
                min.0 = count;
                min.1 = column;
            }
            column_head = self.right[column_head];
        }
        if self.chaining {
            let mut column_head = self.right[1];
            while column_head != 1 {
                let column = self.node_list[column_head].column;
                let count = self.column_list[column].count;
                if count < min.0 {
                    if count == 1 {
                        return Some(column);
                    } else if count > 0 {
                        min.0 = count;
                        min.1 = column;
                    }
                }
                column_head = self.right[column_head];
            }
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

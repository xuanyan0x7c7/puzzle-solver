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
    ConditionalUnique(usize),
    Constraint,
}

struct Column {
    column_type: ColumnType,
    head: usize,
    count: usize,
}

impl Column {
    fn conditional_index(&self) -> Option<usize> {
        match self.column_type {
            ColumnType::ConditionalUnique(index) => Some(index),
            _ => None,
        }
    }
}

struct ConditionState {
    head: usize,
    holes: usize,
    current_holes: usize,
    chaining: bool,
}

enum State {
    New,
    CurrentState(usize, usize),
    TriggerChaining(usize),
}

pub struct PuzzleSolver {
    node_list: Vec<Node>,
    row_list: Vec<Row>,
    column_list: Vec<Column>,
    condition_state_list: Vec<ConditionState>,
    up: Vec<usize>,
    down: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
    state_stack: Vec<State>,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        let mut board = PuzzleSolver {
            node_list: vec![],
            row_list: vec![],
            column_list: vec![],
            condition_state_list: vec![],
            up: vec![],
            down: vec![],
            left: vec![],
            right: vec![],
            state_stack: vec![State::New],
        };
        board.new_head_node();
        board
    }
}

impl Default for PuzzleSolver {
    fn default() -> Self {
        PuzzleSolver::new()
    }
}

impl PuzzleSolver {
    pub fn new_conditional_constraint(&mut self, holes: usize) -> usize {
        let index = self.condition_state_list.len();
        let head = self.new_head_node();
        self.condition_state_list.push(ConditionState {
            head,
            holes,
            current_holes: 0,
            chaining: false,
        });
        index
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
                ColumnType::ConditionalUnique(index) => self.condition_state_list[index].head,
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

    pub fn add_rows(&mut self, row_count: usize) -> usize {
        let column_index = self.column_list.len();
        let column = self.new_column(ColumnType::Unique, row_count);
        let row_index = self.row_list.len();
        for _ in 0..row_count {
            let row = Row {
                head: self.new_node(self.row_list.len(), column_index, usize::MAX, column.head),
                chosen: false,
            };
            self.row_list.push(row);
        }
        self.column_list.push(column);
        row_index
    }

    pub fn add_column<I>(&mut self, rows: I)
    where
        I: ExactSizeIterator + IntoIterator<Item = usize>,
    {
        let column_index = self.column_list.len();
        let column = self.new_column(ColumnType::Unique, rows.len());
        for row in rows {
            self.new_node(row, column_index, self.row_list[row].head, column.head);
        }
        self.column_list.push(column);
    }

    pub fn add_conditional_column<I>(&mut self, rows: I, conditional_index: usize)
    where
        I: ExactSizeIterator + IntoIterator<Item = usize>,
    {
        let column_index = self.column_list.len();
        let column = self.new_column(ColumnType::ConditionalUnique(conditional_index), rows.len());
        for row in rows {
            self.new_node(row, column_index, self.row_list[row].head, column.head);
        }
        self.column_list.push(column);
    }

    pub fn add_constraint<I>(&mut self, rows: I)
    where
        I: ExactSizeIterator + IntoIterator<Item = usize>,
    {
        let column_index = self.column_list.len();
        let column = self.new_column(ColumnType::Constraint, rows.len());
        for row in rows {
            self.new_node(row, column_index, self.row_list[row].head, column.head);
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
                if column_item.count == 0 {
                    if let Some(index) = column_item.conditional_index() {
                        self.condition_state_list[index].current_holes += 1;
                    }
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
        if column_item.count == 0 {
            if let Some(index) = column_item.conditional_index() {
                self.condition_state_list[index].current_holes -= 1;
            }
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
                if column_item.count == 0 {
                    if let Some(index) = column_item.conditional_index() {
                        self.condition_state_list[index].current_holes += 1;
                    }
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
        if column_item.count == 0 {
            if let Some(index) = column_item.conditional_index() {
                self.condition_state_list[index].current_holes += 1;
            }
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
                if column_item.count == 1 {
                    if let Some(index) = column_item.conditional_index() {
                        self.condition_state_list[index].current_holes -= 1;
                    }
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
            match self.state_stack.pop()? {
                State::New => {
                    if self
                        .condition_state_list
                        .iter()
                        .any(|state| state.current_holes > state.holes)
                    {
                        continue;
                    }
                    if self.right[0] == 0 {
                        return Some(
                            self.row_list
                                .iter()
                                .enumerate()
                                .filter_map(
                                    |(index, row)| if row.chosen { Some(index) } else { None },
                                )
                                .collect(),
                        );
                    }
                    for (index, state) in self
                        .condition_state_list
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, state)| !state.chaining && state.current_holes == state.holes)
                    {
                        state.chaining = true;
                        self.state_stack.push(State::TriggerChaining(index));
                    }
                    let Some(best_column) = self.pick_best_column() else {
                        continue;
                    };
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
                        self.state_stack
                            .push(State::CurrentState(best_column, row_node));
                        self.state_stack.push(State::New);
                    }
                }
                State::CurrentState(current_column, current_row_node) => {
                    let mut column_node = self.left[current_row_node];
                    while column_node != current_row_node {
                        self.resume_column(self.node_list[column_node].column);
                        column_node = self.left[column_node];
                    }
                    self.row_list[self.node_list[current_row_node].row].chosen = false;
                    let row_node = self.down[current_row_node];
                    if row_node == self.column_list[current_column].head {
                        self.resume_column(current_column);
                        continue;
                    } else {
                        self.row_list[self.node_list[row_node].row].chosen = true;
                        let mut column_node = self.right[row_node];
                        while column_node != row_node {
                            self.remove_column(self.node_list[column_node].column);
                            column_node = self.right[column_node];
                        }
                        self.state_stack
                            .push(State::CurrentState(current_column, row_node));
                        self.state_stack.push(State::New);
                    }
                }
                State::TriggerChaining(index) => {
                    self.condition_state_list[index].chaining = false;
                }
            }
        }
    }

    fn pick_best_column(&self) -> Option<usize> {
        let mut min_count = usize::MAX;
        let mut min_column = usize::MAX;
        let mut column_head = self.right[0];
        while column_head != 0 {
            let column = self.node_list[column_head].column;
            let count = self.column_list[column].count;
            if count < min_count {
                if count == 1 {
                    return Some(column);
                } else if count == 0 {
                    return None;
                }
                min_count = count;
                min_column = column;
            }
            column_head = self.right[column_head];
        }
        for state in self
            .condition_state_list
            .iter()
            .filter(|state| state.chaining)
        {
            let mut column_head = self.right[state.head];
            while column_head != state.head {
                let column = self.node_list[column_head].column;
                let count = self.column_list[column].count;
                if count < min_count {
                    if count == 1 {
                        return Some(column);
                    } else if count > 0 {
                        min_count = count;
                        min_column = column;
                    }
                }
                column_head = self.right[column_head];
            }
        }
        Some(min_column)
    }
}

pub struct SolverIterator<'a> {
    solver: &'a mut PuzzleSolver,
}

impl<'a> Iterator for SolverIterator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.solve_next()
    }
}

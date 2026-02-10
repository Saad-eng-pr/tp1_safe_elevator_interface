#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor(i32),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

#[derive(Debug, Clone)]
pub struct Elevator {
    pub floor: i32,
    pub state: State,
    pub queue: Vec<i32>,
}

impl Elevator {
    pub fn new(start_floor: i32) -> Result<Self, ElevatorError> {
        if !(0..=5).contains(&start_floor) {
            return Err(ElevatorError::InvalidFloor(start_floor));
        }
        Ok(Self {
            floor: start_floor,
            state: State::Idle,
            queue: vec![],
        })
    }

    pub fn floor(&self) -> i32 {
        self.floor
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn queue(&self) -> &[i32] {
        &self.queue
    }

    pub fn call(&mut self, floor: i32) -> Result<(), ElevatorError> {
        if !(0..=5).contains(&floor) {
            return Err(ElevatorError::InvalidFloor(floor));
        }

        if floor == self.floor || self.queue.contains(&floor) {
            return Ok(());
        }

        self.queue.push(floor);

        if self.state == State::Idle {
            self.state = if floor > self.floor {
                State::MovingUp
            } else {
                State::MovingDown
            };
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ElevatorError> {
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }

        let target = match self.queue.first().copied() {
            Some(t) => t,
            None => {
                self.state = State::Idle;
                return Err(ElevatorError::EmptyQueue);
            }
        };

        if target > self.floor {
            self.floor += 1;
            self.state = State::MovingUp;
        } else if target < self.floor {
            self.floor -= 1;
            self.state = State::MovingDown;
        }

        if self.floor == target {
            self.queue.remove(0);
            self.state = State::DoorsOpen;
        }

        Ok(())
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => Err(ElevatorError::DoorsAlreadyOpen),
            State::MovingDown | State::MovingUp => Err(ElevatorError::CannotOpenWhileMoving),
            State::Idle => {
                self.state = State::DoorsOpen;
                Ok(())
            }
        }
    }

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        match self.state {
            State::DoorsOpen => {
                if let Some(&next) = self.queue.first() {
                    self.state = if next > self.floor {
                        State::MovingUp
                    } else {
                        State::MovingDown
                    };
                } else {
                    self.state = State::Idle;
                }

                Ok(())
            }
            _ => Err(ElevatorError::DoorsAlreadyClosed),
        }
    }

    pub fn status(&mut self) -> Self {
        self.clone()
    }
}

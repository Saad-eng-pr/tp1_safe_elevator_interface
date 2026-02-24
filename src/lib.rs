/// Représente l'état actuel de l'ascenseur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

/// Enumération des erreurs possibles de l'ascenseur.
#[derive(Debug, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor(i32),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

/// Structure représentant un ascenseur.
#[derive(Debug, Clone)]
pub struct Elevator {
    pub floor: i32,
    pub state: State,
    pub queue: Vec<i32>,
}

impl Elevator {
    /// Crée un nouvel ascenseur à l'étage donné.
    ///
    /// # Arguments
    /// * `start_floor` - L'étage de départ (entre 0 et 5).
    ///
    /// # Retour
    /// * `Ok(Elevator)` si l'étage est valide.
    /// * `Err(ElevatorError::InvalidFloor)` sinon.
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

    /// Retourne l'étage actuel de l'ascenseur.
    pub fn floor(&self) -> i32 {
        self.floor
    }

    /// Retourne l'état actuel de l'ascenseur.
    pub fn state(&self) -> State {
        self.state
    }

    /// Retourne la file d'attente des étages à desservir.
    pub fn queue(&self) -> &[i32] {
        &self.queue
    }

    /// Ajoute un appel d'étage à la file d'attente.
    ///
    /// # Arguments
    /// * `floor` - L'étage à appeler (entre 0 et 5).
    ///
    /// # Retour
    /// * `Ok(())` si l'appel est ajouté ou déjà présent.
    /// * `Err(ElevatorError::InvalidFloor)` si l'étage est invalide.
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

    /// Effectue une étape de déplacement de l'ascenseur.
    ///
    /// # Retour
    /// * `Ok(())` si l'ascenseur a bougé ou ouvert les portes.
    /// * `Err(ElevatorError)` si une erreur survient.
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

    /// Ouvre les portes de l'ascenseur si possible.
    ///
    /// # Retour
    /// * `Ok(())` si les portes sont ouvertes.
    /// * `Err(ElevatorError)` si impossible.
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

    /// Ferme les portes de l'ascenseur si possible.
    ///
    /// # Retour
    /// * `Ok(())` si les portes sont fermées.
    /// * `Err(ElevatorError)` si impossible.
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

    /// Retourne une copie de l'état actuel de l'ascenseur.
    pub fn status(&mut self) -> Self {
        self.clone()
    }
}

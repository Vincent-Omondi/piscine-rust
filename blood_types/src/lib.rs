#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            other => Err(format!("`{}` is not a valid antigen", other)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(rhf: &str) -> Result<Self, Self::Err> {
        match rhf {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            o => Err(format!("`{}` is not a valid Rh Factor", o)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rh_factor == other.rh_factor {
            return self.antigen.cmp(&other.antigen);
        }
        self.antigen.cmp(&other.antigen)
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(bt: &str) -> Result<Self, Self::Err> {
        if bt.len() > 3 || bt.len() < 2 {
            return Err(format!(
                "Invalid antigen: `{}` invalid length: {}",
                bt,
                bt.len()
            ));
        }

        let rh_fac_str = bt.get(bt.len() - 1..);

        if let None = rh_fac_str {
            return Err(format!("Invalid suffix {:?}", rh_fac_str));
        }

        let rh_factor = rh_fac_str.unwrap().parse()?;
        let antigen = bt.get(..bt.len() - 1).unwrap().parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.antigen)?;

        if self.rh_factor == RhFactor::Positive {
            return write!(f, "+");
        }

        write!(f, "-")
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Positive can only receive from positive
        // A can only give from A
        // And B can only give to B
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }

        if other.antigen == Antigen::O {
            return true;
        }

        // if self.rh_factor contains one of the antigens of other
        // then it can receive from it
        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    // who are the donors of self
    pub fn donors(&self) -> Vec<Self> {
        // all blood types A, B, AB, O
        let mut blood_types = Vec::new();
        let mut antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        if self.antigen == Antigen::AB {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }

    // who are the recipients of self
    pub fn recipients(&self) -> Vec<BloodType> {
        let mut blood_types = Vec::new();
        let mut antigens = if self.antigen != Antigen::AB {
            vec![Antigen::AB, self.antigen.clone()]
        } else {
            vec![Antigen::AB]
        };

        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Positive, RhFactor::Negative]
        } else {
            vec![RhFactor::Positive]
        };

        if self.antigen == Antigen::O {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }
}
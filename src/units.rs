#[derive(Debug)]
pub enum Operators {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug)]
pub enum Unit {
    // Force
    Newton,
    KiloNewton,
    MegaNewton,
    GigaNewton,
    // Distance
    Millimeter,
    CentiMeter,
    Meter,
    KiloMeter,
    // Area
    SquareMilliMeter,
    SquareCentiMeter,
    SquareMeter,
    SquareKiloMeter,
    // Pressure
    Pascal,
    KiloPascal,
    MegaPascal,
    GigaPascal,
    // Time
    Second,
    Hour,
    // Weight
    Gram,
    KiloGram,
}

impl Unit {
    pub fn to_base_value(&self) -> f64 {
        match self {
            // Force
            Unit::Newton => 1.0,
            Unit::KiloNewton => 1_000.0,
            Unit::MegaNewton => 1_000_000.0,
            Unit::GigaNewton => 1_000_000_000.0,
            // Distance
            Unit::Millimeter => 0.001,
            Unit::CentiMeter => 0.01,
            Unit::Meter => 1.0,
            Unit::KiloMeter => 1_000.0,
            // Area
            Unit::SquareMilliMeter => 0.000_001,
            Unit::SquareCentiMeter => 0.000_1,
            Unit::SquareMeter => 1.0,
            Unit::SquareKiloMeter => 1_000_000.0,
            // Pressure
            Unit::Pascal => 1.0,
            Unit::KiloPascal => 1_000.0,
            Unit::MegaPascal => 1_000_000.0,
            Unit::GigaPascal => 1_000_000_000.0,
            // Time
            Unit::Second => 1.0,
            Unit::Hour => 3_600.0,
            // Weight
            Unit::Gram => 0.001,
            Unit::KiloGram => 1.0,
        }
    }
}

pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
}

pub enum Expr {
    Quantity(Quantity),
    BinOp(Box<Expr>, Operators, Box<Expr>),
}

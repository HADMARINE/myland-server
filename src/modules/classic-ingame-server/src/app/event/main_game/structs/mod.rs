pub mod stats {
    pub struct GlobalGameStat {}
    // pub mod lands
    pub struct User<LandType> {
        pub lands: Vec<Land<LandType>>,
        pub money: f32,
        pub reputation: f32,
        pub political_power: f32,
        pub secretary: Secretary,
        pub loans: Vec<Loan>,
        pub effects: EffectContainer,
    }

    impl<T> User<T> {
        pub fn get_total_population(&self) -> u32 {}
    }

    pub struct Secretary {
        pub name: String,
        pub effects: EffectContainer,
        pub special_effects: SpecialEffect,
    }

    pub struct Advertisement {
        pub name: String,
        pub price: u32,
        pub effect: EffectContainer,
    }

    pub struct EffectContainer {}

    pub struct SpecialEffect {}

    pub enum Component {
        ValueChange(),
    }

    pub enum ValueKind {
        Money,
        Population,
        Attractiveness,
        Productivity,
        PoliticalPower,
    }

    pub struct CustomComponent {
        pub name: String,
        pub details: String,
        pub effect_kind: EffectKind,
        pub amount: f32,
    }

    pub enum EventKind {
        Once,
        Continous,
        Trigger(),
    }

    pub struct Trigger {}

    pub struct Immovables {
        pub building_level: ImmovablesBuildingLevel,
        pub attractiveness: f32,
        pub population_capacity: u32,
        pub productivity: f32,
    }

    pub struct Land<LandType> {
        pub tile_count: u32,
        pub location: LandType, // Put a enum type here
    }

    pub struct Loan {
        pub loan_bank_id: u8, // This is for distinction of ceratin bank
    }
    pub enum ImmovablesBuildingLevel {
        Basic,
        Moderate,
        Advanced,
        Landmark,
    }
}

pub mod status {
    pub struct Territory {
        pub tile_count: u32,
        pub attractiveness: f32,
        pub productivity: f32,
        pub population: u32,
    }
}

pub mod events {
    pub mod cyclic {
        pub mod util {
            pub struct TurnReduction {} // Wrap any events that should be resolved in certain turn

            pub struct Repetitive {} // Wrap any events that should be repetitive
        }

        use crate::app::event::main_game::{cyclic_event_queue::CyclicEvent, structs::stats::User};

        pub struct PersonalEvent<LandType> {
            pub user: User<LandType>,
        }

        impl PersonalEvent {}

        impl CyclicEvent for PersonalEvent {}

        pub struct GlobalEvent {
            // pub
        }

        pub struct Income<LandType> {
            pub user: User<LandType>,
        }

        impl Income {}

        impl CyclicEvent for Income {}

        pub struct Tax {}

        impl CyclicEvent for Tax {}

        pub struct LoanInterest {}

        pub struct LoanPayment {}

        pub struct PopulationTransformation {}

        pub struct SpyRecruit {}

        pub struct SpyActivity {}

        pub struct WinLoseDetermination {}

        pub struct Construction {}

        pub struct Reclamation {}

        pub struct Auction {}

        // 기부 잠금 - user state 로 관리, 변경하는걸 이벤트
        // pub struct
    }
}

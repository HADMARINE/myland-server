pub mod stats {
    pub struct GlobalGameStat {}
    pub mod lands {
        pub enum Seoul {}
    }
    pub struct User<LandType> {
        pub lands: Vec<Land<LandType>>,
        pub money: f32,
        pub reputation: f32,
        pub political_power: f32,
        pub secretary: Secretary<LandType>,
        pub loans: Vec<Loan>,
        pub componenets: Vec<Component<LandType>>,
        pub uuid: String,
    }

    impl<T> User<T> {
        // Getter functions
        pub fn get_total_population(&self) -> u32 {}
        pub fn get_total_productivity(&self) -> u32 {}
        pub fn get_total_attractiveness(&self) -> f32 {}

        // Action
        pub fn allocate_land(&self) {
            // Check the occupation of land via cycling each user
        }

        pub fn transfer_land(&self) {
            // Check that user of self has the ownership of certain land
            // and transfer to certain user
        }
    }

    pub struct Secretary<LandType> {
        pub name: String,
        pub effects: Vec<Component<LandType>>,
        pub special_effects: SpecialEffect,
    }

    pub struct Advertisement<LandType> {
        pub name: String,
        pub description: String,
        pub price: u32,
        pub component: Vec<Component<LandType>>,
    }

    pub struct SpecialEffect {}

    pub struct Component<LandType> {
        pub affected_subject: AffectedSubject<LandType>,
        pub component_kind: ComponentKind,
    }

    impl<LandType> Component<LandType> {
        pub fn resolve(&self) {
            match self.component_kind {
                ComponentKind::ValueChange(value_kind, amount) => {}
                ComponentKind::CustomFunction(func) => {}
            }
        }
    }

    pub enum ComponentKind {
        ValueChange(ValueKind, UnitKind),
        CustomFunction(dyn Fn() -> ()),
    }

    pub enum AffectedSubject<LandType> {
        User(User<LandType>),
        Land(Land<LandType>),
    }

    pub enum UnitKind {
        Percent(f32),
        UnsignedDecimal(u32),
        SignedDecimal(i32),
        Float(f32),
    }

    pub enum ValueKind {
        Money,
        Population,
        Attractiveness,
        Productivity,
        PoliticalPower,
    }

    pub enum EventKind {
        Once,
        Repetitive,
        RepetitiveLimited(u32),
        TurnReduction(u32),
        Trigger(dyn Fn() -> ()),
    }

    pub struct Immovable {
        pub building_level: ImmovablesBuildingLevel,
        pub attractiveness: f32,
        pub population_capacity: u32,
        pub productivity: f32,
        pub required_productivity: f32,
    }

    pub enum ImmovablesBuildingLevel {
        Basic,
        Moderate,
        Advanced,
        Landmark,
    }
    pub struct Land<LandType> {
        pub tile_count: u32,
        pub tile_composition: Vec<Immovable>,
        pub location: LandType, // Put a enum type here
        pub population: u32,
        pub holding_productivity: u32,
        pub currently_building_immovables: (Immovable, u32),
    }

    impl<LandType> Land<LandType> {
        pub fn allocate_land(&self) {}
        pub fn proceed_building_process(&self) {}
        pub fn add_population(&self, population: u32) {}
        pub fn transform_attractiveness_to_population(&self) {}

        // Value getters
        pub fn get_population_capacity(&self) {}
        pub fn get_attractiveness(&self) {}
        pub fn get_total_productivity(&self) {}
    }

    pub struct Loan {
        pub loan_bank_id: u8, // This is for distinction of ceratin bank
        pub loan_amount: f32,
        pub interest_rate: f32,
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

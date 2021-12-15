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

    pub trait Event {
        fn resolve(&self) -> Result<(), Box<dyn std::error::Error>>;
    }

    pub enum ComponentKind {
        ValueChange(ValueKind, UnitKind),
        CustomFunction(Box<dyn Fn() -> ()>),
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
        Reputation,
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
            use crate::app::event::main_game::structs::stats::Event;

            pub struct TurnReduction {
                pub remain_time: u8,
                pub event: Box<dyn Event>,
            } // Wrap any events that should be resolved in certain turn

            pub struct Repetitive {
                pub event: Box<dyn Event>,
            } // Wrap any events that should be repetitive
        }

        use crate::app::event::main_game::{
            cyclic_event_queue::CyclicIntegratedEvent,
            structs::stats::{Event, Land, Loan, User},
        };

        pub struct PersonalEvent<LandType> {
            pub user: User<LandType>,
        }

        impl<LandType> PersonalEvent<LandType> {
            pub fn new(user: &User<LandType>) -> Vec<PersonalEvent<LandType>> {
                let events: Vec<Box<dyn Event>> = Vec::new();
            }
        }

        impl<LandType> CyclicIntegratedEvent for PersonalEvent<LandType> {}

        pub struct GlobalEvent {
            // pub
        }

        pub struct Income<LandType> {
            pub user: User<LandType>,
        }

        impl<LandType> CyclicIntegratedEvent for Income<LandType> {}

        pub struct Tax<LandType> {
            pub user: User<LandType>,
        }

        impl<LandType> CyclicIntegratedEvent for Tax<LandType> {}

        pub struct LoanInterest<LandType> {
            pub user: User<LandType>,
            pub loan: Loan,
        }

        impl<LandType> CyclicIntegratedEvent for LoanInterest<LandType> {}

        pub struct LoanPayment<LandType> {
            pub user: User<LandType>,
            pub loan: Loan,
        }

        impl<LandType> CyclicIntegratedEvent for LoanPayment<LandType> {}

        pub struct PopulationTransformation<LandType> {
            pub land: Land<LandType>,
        }

        pub struct SpyRecruit<LandType> {
            pub user: User<LandType>,
            pub archived_reputation: f32,
        }

        pub struct SpyActivity<LandType> {
            pub event: Box<dyn Event>,
            pub origin_user: User<LandType>,
            pub dest_user: User<LandType>,
            pub dest_land: Land<LandType>,
        }

        impl<LandType> CyclicIntegratedEvent for SpyActivity<LandType> {}

        pub struct WinLoseDetermination {}

        impl CyclicIntegratedEvent for WinLoseDetermination {}

        pub struct Construction<LandType> {
            pub land: Land<LandType>,
            pub tile_index: u8,
        }

        impl<LandType> CyclicIntegratedEvent for Construction<LandType> {}

        pub struct Reclamation<LandType> {
            pub land: Land<LandType>,
            pub tile_index: u8,
        }

        impl<LandType> CyclicIntegratedEvent for Reclamation<LandType> {}

        pub struct Auction<LandType> {
            pub participate_status: Vec<(User<LandType>, f32)>,
            pub land: Land<LandType>,
        }

        impl<LandType> CyclicIntegratedEvent for Auction<LandType> {}

        // 기부 잠금 - user state 로 관리, 변경하는걸 이벤트
        // pub struct
    }
}

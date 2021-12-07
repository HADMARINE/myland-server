pub mod stats {
    pub struct User {

    }
    
    pub struct Immovables {
        pub building_level: Immovables_building_level,
        pub attractiveness: f32,
        pub population_capacity: u32,
        pub productivity: f32,
    }

    pub struct Land {
        pub tile_count: u32,

    }

    pub enum Immovables_building_level {
        Basic,
        Moderate,
        Advanced,
        Landmark

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


        pub struct
    }
}

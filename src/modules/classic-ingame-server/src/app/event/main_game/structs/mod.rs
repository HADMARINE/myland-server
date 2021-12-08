pub mod stats {
    pub mod lands
    pub struct User {

    }
    
    pub struct Immovables {
        pub building_level: ImmovablesBuildingLevel,
        pub attractiveness: f32,
        pub population_capacity: u32,
        pub productivity: f32,
    }

    pub struct Land<T> {
        pub tile_count: u32,
        pub location: T
    }

    

    pub enum ImmovablesBuildingLevel {
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


        // pub struct 
    }
}

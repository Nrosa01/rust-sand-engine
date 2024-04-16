#[cfg(test)]
mod tests {
    use crate::blocks::{Blocks, Number, NumberConstants, NumbersRuntime};

    #[test]
    fn test_block_serialization_and_deserialization() {
        let block1 = Blocks::CompareNumberEquality { block1: Number::NumberConstants(NumberConstants::ParticleIdFromName("Water".to_string())), block2: Number::NumbersRuntime(NumbersRuntime::NumberOfXTouching(NumberConstants::ParticleType(1))) };
        let serialized = serde_json::to_string(&block1).unwrap();
        let deserialized: Blocks = serde_json::from_str(&serialized).unwrap();

        std::fs::write("test.json", serialized).unwrap();
        assert_eq!(block1, deserialized);
    }

    #[test]
    pub fn test_sand_serialization_and_deserialization()
    {
        let blocks = vec![
            Blocks::If { 
                condition: Box::new(Blocks::IfDirectionIsAnyType { 
                    direction: [0, -1], 
                    types: vec![NumberConstants::ParticleIdFromName("empty".to_string()), NumberConstants::ParticleIdFromName("water".to_string())] }), 
                result: Box::new(Blocks::Swap { direction: [0, -1] }), 
                r#else: Some(Box::new(Blocks::If {
                    condition: Box::new(Blocks::IfDirectionIsAnyType { 
                        direction: [-1, -1], 
                        types: vec![NumberConstants::ParticleIdFromName("empty".to_string()), NumberConstants::ParticleIdFromName("water".to_string())] }), 
                    result: Box::new(Blocks::Swap { direction: [-1, -1] }), 
                    r#else: Some(Box::new(Blocks::If {
                        condition: Box::new(Blocks::IfDirectionIsAnyType { 
                            direction: [1, -1], 
                            types: vec![NumberConstants::ParticleIdFromName("empty".to_string()), NumberConstants::ParticleIdFromName("water".to_string())] }), 
                        result: Box::new(Blocks::Swap { direction: [1, -1] }), 
                        r#else: None
                    }))
                }))
            }
        ];
    
    
        println!("Block going to be serilized");
        let serialized = serde_json::to_string(&blocks).map_err(|err| err.to_string()).unwrap();
        println!("Block serilized");
    
        std::fs::write("sand.json", serialized).map_err(|err| err.to_string()).unwrap();
    }
}
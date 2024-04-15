    // Tjos is just a snippet to testing serialization. I should move this to tests at other time
    
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
    let serialized = serde_json::to_string(&blocks).map_err(|err| err.to_string())?;
    println!("Block serilized");

    std::fs::write("sand.json", serialized).map_err(|err| err.to_string())?;

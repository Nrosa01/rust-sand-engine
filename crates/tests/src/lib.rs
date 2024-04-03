#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::time::{Instant, Duration};
    use rustc_hash::FxHashSet;

    #[test]
    fn test_performance_comparison() {
        const N: usize = 1000; 
        const MAX_ELEMENT: usize = 20;

        let mut rng = rand::thread_rng();
        let mut array = [0; MAX_ELEMENT];
        let mut hashset = FxHashSet::default();

        for i in 0..MAX_ELEMENT {
            array[i] = i;
            hashset.insert(i);
        }

        let mut total_array_time = Duration::new(0, 0);
        let mut total_hashset_time = Duration::new(0, 0);

        for _ in 0..N {
            let element_to_find = rng.gen_range(0..MAX_ELEMENT);

            // Array time
            let start_array = Instant::now();
            let _ = array.iter().find(|&&x| x == element_to_find);
            let elapsed_array = start_array.elapsed();
            total_array_time += elapsed_array;

            // HashSet time
            let start_hashset = Instant::now(); 
            let _ = hashset.contains(&element_to_find);
            let elapsed_hashset = start_hashset.elapsed();
            total_hashset_time += elapsed_hashset;
        }

        // Calcular la media de los tiempos
        let average_array_time = total_array_time / N as u32;
        let average_hashset_time = total_hashset_time / N as u32;

        println!("Average time for Array: {:?}", average_array_time);
        println!("Average time for HashSet: {:?}", average_hashset_time);
    }

    const SIZE: usize = 1000; // Tamaño del vector y del array
    const TARGET_ELEMENT: usize = 500; // Elemento a buscar
    const DEFAULT_VALUE: usize = 0; // Valor por defecto para inicializar el array

    #[test]
    fn test_array_vs_vec_performance() {
        let vec: Vec<usize> = (0..SIZE).collect();

        let mut array = [DEFAULT_VALUE; SIZE];
        for i in 0..SIZE {
            array[i] = i;
        }

        // Vector time measurement
        let start_vec = Instant::now();
        let _ = vec.iter().find(|&&x| x == TARGET_ELEMENT);
        let elapsed_vec = start_vec.elapsed();

        // Array time measuremente
        let start_array = Instant::now();
        let _ = array.iter().find(|&&x| x == TARGET_ELEMENT);
        let elapsed_array = start_array.elapsed();

        println!("Time for Vec lookup: {:?}", elapsed_vec);
        println!("Time for Array lookup: {:?}", elapsed_array);
    }
}

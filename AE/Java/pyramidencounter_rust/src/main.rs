//! Pyramiden Zähler

#[derive(Debug, Clone, PartialEq)]
struct Pyramide {
	ebenen: Vec<Ebene>,
}

impl Pyramide {
    fn new(ebenen_zahl: usize) -> Self {
    	let mut ebenen = Vec::new();
    	let mut current = ebenen_zahl;
    		
    	for _ in 0..ebenen_zahl {
            ebenen.push(Ebene::new(current));
        	current -= 1;
    	}
    
    	Self {
    	    ebenen,
    	}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Ebene {
    glaeser_pro_seite: usize,
}

impl Ebene {
    fn new(glaeser_pro_seite: usize) -> Self {
        Ebene {
    	    glaeser_pro_seite,
	    }
    }
}

fn main() {
    let pyramide = Pyramide::new(65);

    let glaeser = glas_counter(pyramide);

    println!("Die Pyramide hat {glaeser} Gläser.");
}

fn glas_counter(pyramid: Pyramide) -> usize {
    let mut sum = 0;

    for i in 0..pyramid.ebenen.len() {
        let count = pyramid.ebenen[i].glaeser_pro_seite;
        sum += count * count;
    }

    return sum;
}

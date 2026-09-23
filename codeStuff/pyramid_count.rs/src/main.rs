//! Pyramiden Zähler

#[derive(Debug, Clone, PartialEq)]
struct Pyramide {
	ebenen: Vec<Ebene>,
}

#[derive(Debug, Clone, PartialEq)]
enum BaseShape {
    Quadrat,    
    Dreieck,    // Gleichseitig
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

    fn glas_counter(&self) -> usize {
        let mut sum = 0;

        for i in 0..self.ebenen.len() {
            let anzahl = self.ebenen[i].glas_counter();
            sum += anzahl;
        }

        return sum;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Ebene {
    glaeser_pro_seite: usize,
    form: BaseShape,
}

impl Ebene {
    fn new(glaeser_pro_seite: usize) -> Self {
        Ebene {
    	    glaeser_pro_seite,
	}
    }

    fn glas_counter(&self) -> usize {
        match self.form {
            Quadrat => {
                let anzahl = self.glaeser_pro_seite.powi(2);
                return anzahl;
            },
            Dreieck => {
                let anzahl = (self.glaeser_pro_seite.powi(2) * 3.sqrt()) / 4;
                return anzahl;
            },
        }
    }
}

fn main() {
    let pyramide = Pyramide::new(65);

    let glaeser = pyramide.glas_counter();
    //let glaeser = glas_counter(pyramide);

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

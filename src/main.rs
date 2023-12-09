
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


struct Symbol { 
    i : usize, 
    j: usize
}

struct Number { 
    i: usize, 
    j: usize, 
    num_digits: usize, 
    n: i32
}


fn parse_line( line: &str, symbols: & mut Vec<Symbol>, lineno: usize,  numbers: & mut Vec<Number> ) { 
    let mut in_digit = false;
    let mut digits: Vec<char> = Vec::new();
    let mut r:usize = 0;
    let mut s:usize = 0;
    for (j,c) in line.chars().enumerate() {
        if c.is_digit(10) { 
            if !in_digit { 
                in_digit = true;
                digits.clear();
                r=lineno; 
                s=j;
            } 
            digits.push(c);
        } else { 
            if in_digit { 
                let ss: String  = digits.iter().collect();
                let n1 = ss.parse::<i32>().unwrap();
                numbers.push( Number{ i: r, j: s, num_digits: digits.len(), n: n1 } );
                digits.clear();
                in_digit = false;
            } ;

            if c!='.' { 
                // must be symbol 
                symbols.push( Symbol{ i:lineno, j:j } );
            }                
        }
    }
    if in_digit { 
        let ss: String  = digits.iter().collect();
        let n1 = ss.parse::<i32>().unwrap();
        numbers.push( Number{ i: r, j: s, num_digits: digits.len(), n: n1 } );
        digits.clear();
        in_digit = false;
    } ;

}



fn parse( lines: & Vec<String> , symbols: & mut Vec<Symbol>, numbers: & mut Vec<Number> ) { 
 
    for (i,l) in lines.iter().enumerate() {
        parse_line(l, symbols, i, numbers);
    }
} 


fn exists_symbol__near_position( i: usize, j: usize, symbols: & Vec<Symbol> ) -> bool { 
    
    for s in symbols{ 
        let di: i64 = (i as i64) - (s.i as i64); 
        let dj: i64 = (j as i64) - (s.j as i64);
        if di.abs()<=1 && dj.abs()<=1 { 
            return true;
        } 
    }
    false
}


fn is_number_near_a_symbol( number: & Number, symbols: & Vec<Symbol> ) -> bool { 

    for s in 0..number.num_digits { 
        if exists_symbol__near_position(number.i, number.j+s, &symbols) { 
            return true
        }
    }
    false
}


fn main() {


    let lines = read_lines("input.txt");
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new(); 

    parse( &lines, &mut symbols, &mut numbers);

    let mut result: usize = 0;

    for n in numbers { 
        if is_number_near_a_symbol(&n, &symbols) { 
            result += n.n as usize;
        }
    }


    println!("Hello, world!");
    println!("Result: {} ", result);
}




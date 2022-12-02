use aoc2022::util::read_line;

// X Rock, Y for Paper, and Z for Scissors
// A Rock, B for Paper, and C for Scissors

#[derive(Debug, PartialEq)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
    Other
}

fn main() {
    let lines = read_line("./input/p2.txt").unwrap();
    let scores:u32 = lines.split("\n").map(|line|{
        let (p1, p2) = line.split_once(" ").unwrap_or(("",""));

        let score1 = match p1.as_bytes(){
            b"A" => Symbol::Rock,
            b"B" => Symbol::Paper,
            b"C" => Symbol::Scissors,
            _ => Symbol::Other
        };

        let score2 = match p2.as_bytes(){
            b"X" => Symbol::Rock,
            b"Y" => Symbol::Paper,
            b"Z" => Symbol::Scissors,
                _ => Symbol::Other 
        };

        let mut score:u32 = 0;
        if score1 == Symbol::Rock && score2 == Symbol::Paper{
            score = 6;
        }
        if score1 == Symbol::Paper && score2 == Symbol::Scissors{
            score = 6;
        }
        if score1 == Symbol::Scissors && score2 == Symbol::Rock{
            score = 6;
        }

        let score_value = match score2{
            Symbol::Rock =>   1,
            Symbol::Paper => 2,
            Symbol::Scissors =>  3,
            _ => 0    
        };

        if score1 != score2{
            score += score_value;
        }else{
            score = score_value*2;
        }
        
    
        println!("{:?} {:?} = win {}",score1,score2, score);
        return score;
    }).sum();
    println!("Total {:?}", scores);

}


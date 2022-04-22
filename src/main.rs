use rand::{Rng};

fn main() {
    let mut win:u32 = 0;
    let mut lose:u32 = 0;

    let mut input = String::new();

    println!("Combien de tests voulez vous faire ?");
    std::io::stdin().read_line(&mut input).unwrap();
    let nb_test: u32 = input.trim().parse().unwrap();

    for _ in 0..nb_test {
        // doors creations
        let doors:[bool; 3] = create_doors();

        // creatiing revealed door and player selection variable
        let reveal;
        let mut sel = rand::thread_rng().gen_range(0..3);

        // vector for storing doors index where goats are
        let mut goats = vec![];
        for (i, _) in doors.iter().enumerate() {
              if doors[i]{
                  continue
              }  else {
                  goats.push(i);
              }
        }

        // revealed door selection
        if sel == goats[0] {
            reveal = goats[1];
        } else if sel == goats[1] {
            reveal = goats[0];
        } else {
            reveal = goats[rand::thread_rng().gen_range(0..2)];
        }

        // change door
        for (i, _) in doors.iter().enumerate() {
            if i == sel {
                continue
            }else if i == reveal {
                continue
            } else {
                sel = i;
                break
            }
        }

        if doors[sel] {
            win += 1;
        } else {
            lose += 1;
        }
    }
    println!("Win : {}, lose : {}", win, lose);


}

fn create_doors() ->[bool; 3] {
    let mut doors = [false, false, false];
    let money = rand::thread_rng().gen_range(0..3);
    doors[money] = true;
    doors
    }
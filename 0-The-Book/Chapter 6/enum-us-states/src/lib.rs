use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum UsStates {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming
}

pub fn state_fact(us_state: UsStates) {
    match us_state {
        // These "facts" were pulled from Bing so they may be incorrect.
        UsStates::Alabama => {
            println!("Alabama is the only state with all major natural resources needed to make \
            iron and steel.")
        }
        UsStates::Alaska => {
            println!("Alaska has the largest area of any U.S. state, and some of its cities and \
            boroughs are larger than some states.")
        }
        UsStates::Arizona => {
            println!("Arizona is famous for the Grand Canyon, which is over a mile deep, 227 miles \
            long and up to 18 miles wide.")
        }
        UsStates::Arkansas => {
            println!("Arkansas is the only US state that produces diamonds.")
        }
        UsStates::California => {
            println!("California is home to the highest and lowest points in the continental U.S.")
        }
        UsStates::Colorado => {
            println!("Colorado is the only state that lies entirely above 1,000 meters in \
            elevation.")
        }
        UsStates::Connecticut => {
            println!("Connecticut is called the “Constitution State” because it was the first \
            state to adopt a written constitution in 1639.")
        }
        UsStates::Delaware => {
            println!("Delaware is the second smallest state in the United States.")
        }
        UsStates::Florida => {
            println!("Florida is the only place in the world where you can find crocodiles and \
            alligators.")
        }
        UsStates::Georgia => {
            println!("Georgia has more soil types than any other state, with over 360 different \
            kinds.")
        }
        UsStates::Hawaii => {
            println!("Surfing was invented in Hawaii.")
        }
        UsStates::Idaho => {
            println!("Idaho has more than 107,000 miles of rivers, the most of any state, and a \
            canyon deeper than the Grand Canyon, called Hells Canyon.")
        }
        UsStates::Illinois => {
            println!("Illinois generates 12% of the nation’s nuclear power and is the \
            fifth-largest energy-consuming state.")
        }
        UsStates::Indiana => {
            println!("Indiana produces more than 20% of the United States’ popcorn supply.")
        }
        UsStates::Iowa => {
            println!("Iowa has the highest literacy rate in the country and is the ice cream \
            capital of the world.")
        }
        UsStates::Kansas => {
            println!("Kansas is flatter than a pancake, according to scientific measurements.")
        }
        UsStates::Kentucky => {
            println!("Kentucky is the only U.S. state to have a continuous border of water \
            running along three of its sides.")
        }
        UsStates::Louisiana => {
            println!("Louisiana is named after King Louis XIV of France.")
        }
        UsStates::Maine => {
            println!("Maine has the highest moose population among the lower 48 states and the \
            most offshore islands.")
        }
        UsStates::Maryland => {
            println!("Maryland was named after Queen Henrietta Maria of England.")
        }
        UsStates::Massachusetts => {
            println!("Massachusetts was the site of America’s first post office, the Salem witch \
            trials, and the first telephone demonstration.")
        }
        UsStates::Michigan => {
            println!("Michigan was the first European settlement in the Midwest, Sault Ste. Marie.")
        }
        UsStates::Minnesota => {
            println!("Minnesota has over 69,000 miles of rivers and streams, and more than \
            10,000 lakes.")
        }
        UsStates::Mississippi => {
            println!("Mississippi was the first state to have a planned system of junior colleges \
            and the 20th state to join the union.")
        }
        UsStates::Missouri => {
            println!("Missouri is home to the biggest mammal in North America, the American bison, \
            and other wildlife such as black bears, bobcats, mountain lions, and gray wolves.")
        }
        UsStates::Montana => {
            println!("Montana is the only state with a triple divide, which allows water to flow \
            into the Pacific Ocean, Atlantic Ocean, and the Hudson Bay.")
        }
        UsStates::Nebraska => {
            println!("Nebraska has the world’s largest hand-planted forest and the largest \
            mammoth fossil.")
        }
        UsStates::Nevada => {
            println!("Nevada has more mountain ranges than any other state in the U.S.")
        }
        UsStates::NewHampshire => {
            println!("New Hampshire was the first American colony to declare its independence from \
            Great Britain and to create its own constitution.")
        }
        UsStates::NewJersey => {
            println!("New Jersey has the highest density population of any state in the U.S. and \
            90% of its people live in urban areas.")
        }
        UsStates::NewMexico => {
            println!("New Mexico is famous for its wineries and wine growing, producing millions \
            of gallons per year since the late 1900s.")
        }
        UsStates::NewYork => {
            println!("New York was called New Amsterdam initially when it was discovered and \
            settled by the Dutch.")
        }
        UsStates::NorthCarolina => {
            println!("In North Carolina, the Outer Banks are nicknamed “Graveyard of the Atlantic” \
             and is where Pirate Blackbeard eventually died due to rough storms.")
        }
        UsStates::NorthDakota => {
            println!("North Dakota has the highest percentage of church-going population in the \
            country, and it also has more churches per capita than any other state.")
        }
        UsStates::Ohio => {
            println!("Ohio once went to war against Michigan over a strip of land.")
        }
        UsStates::Oklahoma => {
            println!("An Oklahoma resident named Gordon Matthews invented the Voice Mail Service.")
        }
        UsStates::Oregon => {
            println!("Oregon has more ghost towns than any other state.")
        }
        UsStates::Pennsylvania => {
            println!("Pennsylvania is the first state to list their web site URL on a license \
            plate.")
        }
        UsStates::RhodeIsland => {
            println!("Rhode Island became America’s first industrialized state.")
        }
        UsStates::SouthCarolina => {
            println!("South Carolina produces more peaches and collard greens than any other state \
            except California")
        }
        UsStates::SouthDakota => {
            println!("South Dakota leads the nation in production of bison and pheasants.")
        }
        UsStates::Tennessee => {
            println!("Tennessee is the origin of Mountain Dew, Krystal Hamburgers and MoonPies, \
            popular food and drink items.")
        }
        UsStates::Texas => {
            println!("In Texas, Vietnamese and Chinese are among the most spoken languages.")
        }
        UsStates::Utah => {
            println!("The federal government owns 65% of Utah’s land.")
        }
        UsStates::Vermont => {
            println!("No city has over 50,000 residents in Vermont.")
        }
        UsStates::Virginia => {
            println!("Virginia is known as the “Mother of Presidents” because eight U.S. \
            presidents were born there, more than any other state.")
        }
        UsStates::Washington => {
            println!("There is a town in Washington that is camouflaged by a fake town on top of \
            it to hide a Boeing factory.")
        }
        UsStates::WestVirginia => {
            println!("West Virginia has one of the oldest and most obese populations in the \
            nation.")
        }
        UsStates::Wisconsin => {
            println!("Margarine was illegal in Wisconsin until 2011.")
        }
        UsStates::Wyoming => {
            println!("Wyoming is the least populated state in the U.S.")
        }
    }
}
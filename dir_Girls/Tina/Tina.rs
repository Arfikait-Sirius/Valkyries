//------------------------
// :[ NAME ]:
//     fn_set_girl_name
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_set_girl_name( girl_name: String ) -> () {
    println!( "{}{}", sp_get_girls_name_label(), girl_name ) ;

    return ;
}

//------------------------
// :[ NAME ]:
//     fn_set_skill_name
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_set_skill_name( target: String ) -> () {
    println!( "{}{}()", sp_get_target_label(), target ) ;

    return ;
}

//------------------------
// :[ NAME ]:
//     fn_judge
//
// :[ CATEGORY ]:
//     Skill
//------------------------
#[allow( dead_code )]
pub fn fn_judge( judgement: bool, target: String ) -> () {

    if judgement {
        println!( "{}OK", sp_get_judgement_label() ) ;
    }else{
        println!( "{}NG - Please check \"{}\".", sp_get_judgement_label(), target ) ;
    }

    return ;
}

fn sp_get_girls_name_label() -> String {
    return "[GIRLS-NAME]: ".to_string() ;
}
fn sp_get_target_label() -> String {
    return "[    TARGET]: ".to_string() ;
}
fn sp_get_judgement_label() -> String {
    return "[     JUDGE]: ".to_string() ;
}
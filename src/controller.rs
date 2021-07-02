use crate::PieceGen;
use crate::tetris::Board;

//tous les éléments sont publics
pub enum ResultController{
    RightBorder,
    LeftBorder,
    BottomBorder,
    Ok,
}

//les différentes actions possibles d'un joueur sur un jeu.
pub enum Action<'a, T, Board>
where T: PieceGen {
    Right(T, &'a Board),
    Bottom(T, &'a Board),
    Left(T, &'a Board),
}


pub trait Controller{
    //on ne va pas baser notre ui sur des signaux. Donc ca na pas de sens
    fn check<T: PieceGen + Iterator>(action : Action<T, Board>)->ResultController{
	match action{
	    Action::Right(piece, board)=>{
		//box is a keyword in rust
		for point in piece.get_points(){
		    if point.x + 1 >= crate::WIDTH {
			return ResultController::RightBorder;
		    }
		}
	    },

	    Action::Left(piece, board)=>{
		//box is a keyword in rust
		for point in piece.get_points(){
		    if point.x - 1 < 0 {
			return ResultController::RightBorder;
		    }
		}
	    },
	    
	    Action::Bottom(piece, board) =>{
		for point in piece.get_points(){
		    if point.y + 1 >= crate::HEIGHT {
			return  ResultController::BottomBorder;
		    }
		}
	    }
	}
	ResultController::Ok
    }
}

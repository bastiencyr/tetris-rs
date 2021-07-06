use crate::PieceModel;
use crate::tetris::Board;

//tous les éléments sont publics
pub enum ResultController{
    RightBorder,
    LeftBorder,
    BottomBorder,
    Ok,
    CollisionPiece,
    CollisionPieceBottom,
}

//les différentes actions possibles d'un joueur sur un jeu.
pub enum Action<'a, T, Board>
where T: PieceModel {

    Right(T, &'a Board),
    Bottom(T, &'a Board),
    Left(T, &'a Board),
}


pub trait Controller{
   
    fn check<T: PieceModel + Iterator>(action : Action<T, Board>)->ResultController{
	match action{
	    Action::Right(piece, board)=>{
		//box is a keyword in rust
		for point in piece.get_points(){
		    if point.x + 1 >= crate::WIDTH {
			return ResultController::RightBorder;
		    }
		    if board.get_i_j(point.x + 1, point.y).empty() == false{
			return ResultController::CollisionPiece;
		    }
		}
	    },
	    
	    Action::Left(piece, board)=>{
		//box is a keyword in rust
		for point in piece.get_points(){
		    if point.x - 1 < 0 {
			return ResultController::RightBorder;
		    }
		    if board.get_i_j(point.x - 1, point.y).empty() == false{
			return ResultController::CollisionPiece;
		    }
		}
	    },
	    
	    Action::Bottom(piece, board) =>{
		for point in piece.get_points(){
		    if point.y + 1 >= crate::HEIGHT {
			return  ResultController::BottomBorder;
		    }
		    if board.get_i_j(point.x, point.y + 1).empty() == false{
			return ResultController::CollisionPieceBottom;
		    }
		}
	    }
	    
	}
	ResultController::Ok
    }
}

use poem::http::StatusCode;
use poem::middleware::AddData;
use poem::web::{Data, Path};
use poem::{get, handler, post, Endpoint, EndpointExt, IntoResponse, Route};
use rand::{Rng, SeedableRng};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]
enum Token {
    #[serde(rename = "cookie")]
    Cookie,
    #[serde(rename = "milk")]
    Milk,
    Empty,
}

impl From<Token> for char {
    fn from(value: Token) -> Self {
        match value {
            Token::Cookie => '🍪',
            Token::Milk => '🥛',
            Token::Empty => '⬛',
        }
    }
}

struct BoardImpl {
    inner: [[Token; 4]; 4],
    rng: rand::rngs::StdRng,
}

impl BoardImpl {
    fn new() -> Self {
        Self {
            inner: [[Token::Empty; 4]; 4],
            rng: rand::rngs::StdRng::seed_from_u64(2024),
        }
    }

    fn print(&self) -> String {
        let mut result = String::new();

        for row in self.inner {
            // Add the left border.
            result.push('⬜');
            for token in row {
                result.push(token.into());
            }
            // Add the right border.
            result.push('⬜');
            result.push('\n');
        }

        // Add the bottom border.
        result.push_str("⬜⬜⬜⬜⬜⬜\n");
        match self.check_winner() {
            Some(winner @ (Token::Cookie | Token::Milk)) => {
                result.push(winner.into());
                result.push_str(" wins!\n");
            }
            Some(Token::Empty) => result.push_str("No winner.\n"),
            None => {}
        };
        result
    }

    fn place(&mut self, pos: usize, token: Token) {
        for i in (0..self.inner.len()).rev() {
            if matches!(self.inner[i][pos], Token::Empty) {
                self.inner[i][pos] = token;
                return;
            }
        }
    }

    fn check_winner(&self) -> Option<Token> {
        if !matches!(self.inner[0][0], Token::Empty)
            && self.inner[0][0] == self.inner[1][1]
            && self.inner[1][1] == self.inner[2][2]
            && self.inner[2][2] == self.inner[3][3]
        {
            // Diagonal top left to bottom right (easier to hardcode for this case)
            Some(self.inner[0][0])
        } else if !matches!(self.inner[3][0], Token::Empty)
            && self.inner[3][0] == self.inner[2][1]
            && self.inner[2][1] == self.inner[1][2]
            && self.inner[1][2] == self.inner[0][3]
        {
            // Diagonal bottom left to top right (easier to hardcode for this case)
            Some(self.inner[3][0])
        } else if let Some(winner) = self.inner.iter().find(|x| {
            x.iter()
                .all(|&token| token == x[0] && !matches!(token, Token::Empty))
        }) {
            // Horizontal check
            Some(winner[0])
        } else if let Some(winner) = (0..self.inner.len()).find(|&i| {
            !matches!(self.inner[0][i], Token::Empty)
                && self.inner[0][i] == self.inner[1][i]
                && self.inner[1][i] == self.inner[2][i]
                && self.inner[2][i] == self.inner[3][i]
        }) {
            // Vertical check
            Some(self.inner[0][winner])
        } else if self
            .inner
            .iter()
            .all(|x| x.iter().all(|&token| !matches!(token, Token::Empty)))
        {
            // No one won
            Some(Token::Empty)
        } else {
            // Not yet a winner
            None
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

type Board = Arc<Mutex<BoardImpl>>;

#[handler]
fn show_board(Data(board): Data<&Board>) -> String {
    board.lock().unwrap().print()
}

#[handler]
fn reset_board(Data(board): Data<&Board>) -> String {
    let mut board = board.lock().unwrap();
    board.reset();
    board.print()
}

#[derive(Deserialize, Debug)]
struct PlaceBoard {
    team: Token,
    position: usize,
}

#[handler]
fn place_board(Path(pb): Path<PlaceBoard>, Data(board): Data<&Board>) -> impl IntoResponse {
    let mut board = board.lock().unwrap();
    if board.check_winner().is_some() {
        StatusCode::SERVICE_UNAVAILABLE
    } else if pb.position >= 5 || pb.position == 0 {
        StatusCode::BAD_REQUEST
    } else {
        board.place(pb.position - 1, pb.team);

        StatusCode::OK
    }
    .with_body(board.print())
}

#[handler]
fn random_board(Data(board): Data<&Board>) -> String {
    let mut board = board.lock().unwrap();

    for i in 0..4 {
        for j in 0..4 {
            board.inner[i][j] = if board.rng.gen::<bool>() {
                Token::Cookie
            } else {
                Token::Milk
            };
        }
    }

    board.print()
}

pub(crate) fn route() -> impl Endpoint {
    Route::new()
        .at("/board", get(show_board))
        .at("/reset", post(reset_board))
        .at("/place/:team/:position", post(place_board))
        .at("/random-board", get(random_board))
        .with(AddData::new(Arc::new(Mutex::new(BoardImpl::new()))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vertical_win() {
        let mut board = BoardImpl::new();

        board.place(0, Token::Cookie);
        board.place(0, Token::Cookie);
        board.place(0, Token::Cookie);
        board.place(0, Token::Cookie);

        assert_eq!(board.check_winner(), Some(Token::Cookie));
    }
}

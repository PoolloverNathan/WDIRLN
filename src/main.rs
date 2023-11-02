mod creatures;
mod list;

use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::{io, ops};

use anyhow::Context as _;
use creatures::Creature;
use list::{List, ListOf};
use ncursesw::{normal::*, *};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Terrain {
  Wall,
  Floor,
  Door { vertical: bool, open: bool },
}

macro_rules! defcolor {
    ($name:ident: $fg:ident / $bg:ident) => {
        let $name = alloc_pair(Colors::new(Color::new(ColorPalette::$fg), Color::new(ColorPalette::$bg))).context(concat!("while allocating color pair ", stringify!($name)))?;
        let $name = &$name;
    };
    ($name:ident: $fg:ident) => {
        defcolor!($name: $fg / Black);
    };
}
macro_rules! cattri {
  ($c:ident) => {
    $c | Attribute::Normal
  };
}
macro_rules! oat {
  ($x:expr, $y:expr) => {
    Origin { x: $x, y: $y }
  };
}
trait CharType {
  type Chtype;

  fn chtype(self) -> Self::Chtype;
}
trait ApplyStyle
where
  Self: Sized,
{
  type Styled;

  fn style(self, attrs: Attributes, color: &ColorPair) -> Self::Styled;
  fn style_with(self, attr: Attribute) -> Self::Styled {
    self.style(Attributes::default() | attr, &ColorPair::default())
  }
  fn paint(self, color: &ColorPair) -> Self::Styled {
    self.style(Attributes::default(), color)
  }
}

impl CharType for u8 {
  type Chtype = ChtypeChar;

  fn chtype(self) -> Self::Chtype {
    extern crate ascii;

    ChtypeChar::new(
      ascii::AsciiChar::from_ascii(self).expect("Internal error: invalid ASCII character"),
    )
  }
}

impl CharType for &[u8] {
  type Chtype = ChtypeString;

  fn chtype(self) -> Self::Chtype {
    extern crate ascii;

    let mut str = ChtypeString::with_capacity(self.len());
    for i in self {
      str.push(i.chtype())
    }
    str
  }
}

impl<I> ApplyStyle for I
where
  I: CharType,
  I::Chtype: ops::BitOr<Attributes, Output = I::Chtype>,
{
  type Styled = I::Chtype;

  fn style(self, attrs: Attributes, color: &ColorPair) -> Self::Styled {
    self.chtype() | attrs
  }
}

impl ApplyStyle for char {
  type Styled = ComplexChar;

  fn style(self, attrs: Attributes, color: &ColorPair) -> Self::Styled {
    ComplexChar::from_char(self, &attrs, color).expect("Failed to style char")
  }
}

impl ApplyStyle for &str {
  type Styled = ComplexString;

  fn style(self, attrs: Attributes, color: &ColorPair) -> Self::Styled {
    ComplexString::from_str(self, &attrs, color).expect("Failed to style char")
  }
}

enum Death {
  Dying,
}

trait All {}
impl<T> All for T {}

macro_rules! termion_init {
    ($($func:ident)::+ [$c:expr] $err:literal) => {
        $c.push(
            Box::new(
                termion::$($func)::+(io::stdout()).context(concat!("Failed to ", $err))?
            )
        );
    }
}

#[derive(Clone, Debug, Default)]
struct Todo(Option<Cow<'static, str>>);
const TODO: Todo = Todo(None);
impl Display for Todo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      Some(why) => write!(f, "Not yet implemented: {}", why),
      None => write!(f, "Not yet implemented"),
    }
  }
}
impl Error for Todo {}

macro_rules! todo {
  () => {
    Err(Todo(None))?
  };
  ($why:expr) => {
    Err(Todo(Some(Cow::from($why))))?
  };
}

struct GameState<'a> {
  gc: &'a mut Vec<Box<dyn All>>,
  map: [[Terrain; 16]; 16],
}

fn game(state: GameState) -> anyhow::Result<()> {
  let GameState { ref mut gc, ref mut map }
  termion_init!(screen::IntoAlternateScreen::into_alternate_screen [state.gc] "enter altscreen");
  initscr().context("Failed to initialize terminal")?;
  curs_set(CursorType::Invisible).context("Failed to hide cursor")?;
  keypad(stdscr(), true).context("Failed to enable keybinds")?;
  start_color().context("Failed to start colors")?;
  defcolor!(stdwall: White);
  defcolor!(stdfloor: LightBlack);
  defcolor!(stdhum: LightWhite);
  defcolor!(stddoor: Yellow);

  for i in 0..16 {
    map[i][0] = Terrain::Wall;
    map[i][15] = Terrain::Wall;
    map[0][i] = Terrain::Wall;
    map[15][i] = Terrain::Wall;
  }
  let (mut player_x, mut player_y) = (1, 1);

  loop {
    match tick() {
      Ok(Ok(true)) => draw().context("While drawing world")?,
      Ok(Ok(false)) => {},
      Ok(Err(why)) => todo!("death handling"),
      Err(why) => return Err(why.context("While ticking world")),
    }
  }

  Ok(())
}

/// Runs ticks for the world and all creatures, including the player.
/// Returns true if the world should be redrawn.
fn tick(state: &mut GameState) -> anyhow::Result<Result<bool, Death>> {
  // terrain ticking isn't implemented yet
  Ok(())
}

fn mkstate(gc: &mut Vec<Box<dyn All>>) -> GameState {
  GameState {
    gc,
    map: [[Terrain::Floor; 16]; 16],
  }
}

fn draw(state: &GameState) -> anyhow::Result<()> {
  todo!();
}

use anyhow::{anyhow, bail};
use termion::screen::AlternateScreen;

fn main() -> anyhow::Result<()> {
  let mut cleanup = Vec::with_capacity(2);
  let out = match (game(&mut cleanup), endwin()) {
    (Ok(()), Ok(())) => Ok(()),
    (Err(e), Ok(())) => Err(e),
    (Ok(()), Err(e)) => Err(anyhow!(e).context("Failed to kill ncurses")),
    (Err(ge), Err(ce)) => Err(anyhow!("{ge}\n\nSuppressed by:\n{ce}")),
  };
  drop(cleanup);
  out
}

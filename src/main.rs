use std::{collections::HashMap, ops::{Add, AddAssign}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ManaType {
    W,
    U,
    B,
    R,
    G,
    C
}

pub use ManaType::*;

impl ManaType {
    const fn usize(self) -> usize {
        match self {
            W => 0,
            U => 1,
            B => 2,
            R => 3,
            G => 4,
            C => 5
        }
    }
}

#[derive(Copy, Clone)]
pub struct ManaCost(
    [usize; 6]
);

#[derive(Copy, Clone)]
pub struct ManaSet(
    [usize; 6]
);

impl Add<ManaSet> for ManaSet {
    type Output = Self;

    fn add(self, rhs: ManaSet) -> Self::Output {
        Self (
            [
                self.0[0] + rhs.0[0],
                self.0[1] + rhs.0[1],
                self.0[2] + rhs.0[2],
                self.0[3] + rhs.0[3],
                self.0[4] + rhs.0[4],
                self.0[5] + rhs.0[5],
            ]
        )
    }
}

impl AddAssign<ManaSet> for ManaSet {
    fn add_assign(&mut self, rhs: ManaSet) {
        for i in 0..6 {
            self.0[i] += rhs.0[i]
        }
    }
}

impl std::fmt::Display for ManaSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for val in self.0.iter() {
            write!(f, "{} ", val)?;
        }
        write!(f, "]")
    }
}

impl std::fmt::Display for ManaCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for val in self.0.iter() {
            write!(f, "{} ", val)?;
        }
        write!(f, "]")
    }
}

impl ManaSet {
    fn contains(&self, o: &ManaCost) -> bool {
        self.0[0] >= o.0[0] &&
        self.0[1] >= o.0[1] &&
        self.0[2] >= o.0[2] &&
        self.0[3] >= o.0[3] &&
        self.0[4] >= o.0[4] &&
        self.0.iter().sum::<usize>() >= o.0.iter().sum::<usize>()
    }
    fn empty() -> Self {
        Self([0;6])
    }
    const fn s(t: ManaType) -> Self {
        let mut o = Self([0;6]);
        o.0[t.usize()] = 1;
        o
    }
}

trait Card {
    fn mana_gen(&self) -> Option<&'static [ManaSet]>;
    fn mana_gen_fast(&self) -> Option<&'static [ManaSet]>;
    fn is(&self, name: &str) -> bool {
        name == self.name()
    }
    fn name(&self) ->  &str;
}

pub struct Land {
    name: &'static str,
    types: &'static [ManaSet]
}

impl Card for Land {
    fn mana_gen(&self) -> Option<&'static [ManaSet]> {
        Some(self.types)
    }

    fn mana_gen_fast(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn name(&self) ->  &str {
        self.name
    }
}

pub struct Mox {
    name: &'static str,
    types: &'static [ManaSet]
}

impl Card for Mox {
    fn mana_gen(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn mana_gen_fast(&self) -> Option<&'static [ManaSet]> {
        Some(self.types)
    }
    
    fn name(&self) ->  &str {
        self.name
    }
}

pub struct Dud {}
impl Card for Dud {
    fn mana_gen(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn mana_gen_fast(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn name(&self) ->  &str {
        "Dud"
    }
}

const MANA_LOTUS: &'static [ManaSet] = &[
    ManaSet([3,0,0,0,0,0]),
    ManaSet([0,3,0,0,0,0]),
    ManaSet([0,0,3,0,0,0]),
    ManaSet([0,0,0,3,0,0]),
    ManaSet([0,0,0,0,3,0]),
];

const MANA_W: &'static [ManaSet] = &[ManaSet::s(W)];
const MANA_U: &'static [ManaSet] = &[ManaSet::s(U)];
const MANA_B: &'static [ManaSet] = &[ManaSet::s(B)];
const MANA_R: &'static [ManaSet] = &[ManaSet::s(R)];
const MANA_G: &'static [ManaSet] = &[ManaSet::s(G)];

const MANA_RG: &'static [ManaSet] = &[
    ManaSet::s(R),
    ManaSet::s(G)
];
const MANA_CITY: &'static [ManaSet] = &[
    ManaSet::s(W),
    ManaSet::s(B),
    ManaSet::s(U),
    ManaSet::s(R),
    ManaSet::s(G)
];

#[derive(Clone, Copy)]
pub struct BlackLotus{}
impl Card for BlackLotus {
    fn mana_gen(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn mana_gen_fast(&self) -> Option<&'static [ManaSet]> {
        Some(MANA_LOTUS)
    }

    fn is(&self, name: &str) -> bool {
        name == "Black Lotus"
    }
    
    fn name(&self) ->  &str {
        "Black Lotus"
    }
}

#[derive(Clone, Copy)]
pub struct NonMana {
    name: &'static str
}

impl Card for NonMana {
    fn mana_gen(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn mana_gen_fast(&self) -> Option<&'static [ManaSet]> {
        None
    }

    fn is(&self, name: &str) -> bool {
        name == self.name
    }
    
    fn name(&self) ->  &str {
        self.name
    }
}
pub struct Hand {
    cards: Vec::<&'static dyn Card>
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.cards.iter() {
            write!(f, "{},", c.name())?;
        }
        Ok(())
    }
}
impl Hand {
    fn has_cards(&self, names: &[(&str, usize)]) -> bool {
        for pair in names.iter() {
            if self.cards.iter().filter(|c| c.is(pair.0)).count() < pair.1 {
                return false;
            }
        }
        true
    }

    fn can_produce(&self, mana: &ManaCost) -> bool {
        self.can_produce_rec(mana, ManaSet::empty(), 0, true)
    }

    fn can_produce_rec(&self, goal: &ManaCost, curr: ManaSet, idx: usize, can_land: bool) -> bool {
        if idx >= self.cards.len() {
            return false;
        }
        for (val, needs_land) in [(self.cards[idx].mana_gen(), true), (self.cards[idx].mana_gen_fast(), false)].iter() {
            let Some(sets) = val else {
                continue
            };
            if *needs_land && !can_land {
                continue;
            }
            for newset in sets.iter() {
                let sum = curr + *newset;
                if sum.contains(goal) {
                    return true;
                }
                for newidx in idx+1..self.cards.len() {
                    if self.can_produce_rec(goal, sum, newidx, if *needs_land { false } else { can_land }) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub struct Deck {
    cards: Vec::<&'static dyn Card>,
    deck: HashMap::<usize, usize>
}

#[allow(unused)]
fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn min(a: usize, b: usize) -> usize {
    if a > b { b } else { a }
}

impl Deck {
    fn num_cards(&self) -> usize {
        self.deck.values().sum()
    }
    fn deal_hand(&self, cards: &[usize]) -> Hand {
        let mut h = Hand {cards: Vec::new()};
        for c_idx in cards.iter() {
            h.cards.push(self.cards[*c_idx]);
        }
        h
    }

    #[allow(unused)]
    fn name_by_idx(&self, idx: usize) -> &str {
        self.cards[idx].name()
    }

    #[allow(unused)]
    fn count_hands<const N: usize>(&self) -> usize {
        let mut cards: [usize; N] = [0; N];
        self.count_hands_rec(&mut cards, 0, 0, |_| true)
    }

    fn count_hands_if<const N: usize>(&self,  pred: fn(&Hand) -> bool) -> usize {
        let mut cards: [usize; N] = [0; N];
        self.count_hands_rec(&mut cards, 0, 0, pred)
    }

    fn count_hands_rec<const N: usize>(&self, cards: &mut [usize; N], dealt: usize, deck_idx: usize, pred: fn(&Hand) -> bool) -> usize {
        if dealt == N {
            let hand = self.deal_hand(cards);
            if !pred(&hand) {
                return 0;
            }
            return self.count(&cards);
        }
        let mut count = 0;
        for card_idx in deck_idx..self.cards.len() {
            let left = self.deck.get(&card_idx).unwrap_or(&0);
            if *left == 0 {
                continue;
            }
            
            for deal in 1..=min(*left, N - dealt) {
                for i in 0..deal {
                    cards[dealt + i] = card_idx;
                }
                count += self.count_hands_rec(cards, dealt + deal, card_idx + 1, pred);
            }
        }
        count
    }

    fn count<const N: usize>(&self, cards: &[usize; N]) -> usize {
        let mut count = 1;
        let mut idx = 0;
        while idx < N {
            let card = cards[idx];
            let mut ct = 1;
            while idx < N-1 && cards[idx+1] == card {
                idx += 1;
                ct += 1;
            }
            let cardct = self.deck.get(&card).unwrap();
            
            count *= ncr(*cardct, ct);
            idx += 1;
        }
        return count;
    }
    
    fn empty() -> Self {
        Self {
            cards: Vec::new(),
            deck: HashMap::new(),
        }
    }

    fn add(&mut self, c: &'static dyn Card, ct: usize) {
        self.cards.push(c);
        self.deck.insert(self.cards.len() - 1, ct);
    }
}

fn fac(n: usize) ->  usize {
    if n == 1 {
        return 1;
    }
    return n * fac(n-1);
}

fn ncr(n: usize, r: usize) -> usize {
    if n == 0 || r == 0 {
        return 0;
    }
    if n == r {
        return 1;
    }
    if r > n-r {
        let mut num = 1;
        for i in r+1..=n {
            num *= i;
        }
        return num / fac(n-r);
    }
    let mut num = 1;
    for i in (n-r)+1..=n {
        num *= i;
    }
    return num / fac(r);
}

pub const LOTUS: BlackLotus = BlackLotus{};
pub const CITY:  Land = Land { name: "City of Brass", types: MANA_CITY };
pub const TAIGA:  Land = Land { name: "Taiga", types: MANA_RG };
pub const BALL: NonMana = NonMana { name: "Ball Lightning" };
pub const BERSERK: NonMana = NonMana { name: "Berserk" };
pub const BLOODLUST: NonMana = NonMana { name: "Bloodlust" };
pub const MOUNTAIN: Land = Land { name: "Mountain", types: MANA_R };
pub const FOREST: Land = Land { name: "Forest", types: MANA_G };
pub const MOX_R: Mox = Mox { name: "Mox Ruby", types: MANA_R };
pub const MOX_G: Mox = Mox { name: "Mox Emerald", types: MANA_G };
pub const MOX_U: Mox = Mox { name: "Mox Sapphire", types: MANA_U };
pub const MOX_W: Mox = Mox { name: "Mox Pearl", types: MANA_W };
pub const MOX_B: Mox = Mox { name: "Mox Jet", types: MANA_B };
pub const DUD: Dud = Dud{};

fn main() {
    fn is_double_berserk(h: &Hand) -> bool {
        if !h.has_cards(&[("Ball Lightning", 1), ("Berserk", 2)]) {
            return false;
        }
        if !h.can_produce(&ManaCost([0, 0, 0, 3, 2, 0])) {
            return false;
        }
        true
    }
    fn is_bloodlust_berserk(h: &Hand) -> bool {
        if !h.has_cards(&[("Ball Lightning", 1), ("Berserk", 1), ("Bloodlust", 1)]) {
            return false;
        }
        if !h.can_produce(&ManaCost([0, 0, 0, 4, 1, 1])) {
            return false;
        }
        true
    }
    let mut d = Deck::empty();
    d.add(&LOTUS, 1);
    d.add(&BALL, 4);
    d.add(&BERSERK, 4);
    d.add(&TAIGA, 4);
    d.add(&CITY, 4);
    d.add(&MOX_R, 1);
    d.add(&MOX_G, 1);
    d.add(&MOX_W, 1);
    d.add(&MOX_U, 1);
    d.add(&MOX_B, 1);
    d.add(&MOUNTAIN, 4);
    d.add(&FOREST, 4);
    d.add(&BLOODLUST, 4);
    d.add(&DUD, 26);
    assert!(d.num_cards() == 60);
    println!("Double Berserk: {}", d.count_hands_if::<7>(is_double_berserk));
    println!("Bloodlust + Berserk: {}", d.count_hands_if::<7>(is_bloodlust_berserk));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_tests() {
        let hand = Hand {
            cards: vec![(&LOTUS)],
        };
        assert!(hand.can_produce(&ManaCost([2, 0, 0, 0, 0, 0])));
        assert!(hand.can_produce(&ManaCost([0, 3, 0, 0, 0, 0])));
        assert!(hand.can_produce(&ManaCost([0, 0, 1, 0, 0, 0])));
        assert!(hand.can_produce(&ManaCost([0, 0, 0, 2, 0, 1])));
        assert!(!hand.can_produce(&ManaCost([1, 1, 0, 0, 0, 0])));
        assert!(!hand.can_produce(&ManaCost([1, 0, 0, 0, 0, 3])));
        let hand = Hand {
            cards: vec![
                (&LOTUS),
                (&CITY)
            ],
        };
        assert!(hand.has_cards(&[("Black Lotus", 1), ("City of Brass", 1)]));
        assert!(!hand.has_cards(&[("Black Lotus", 2), ("City of Brass", 1)]));
        assert!(hand.has_cards(&[("City of Brass", 1)]));
        assert!(!hand.has_cards(&[("Mountain", 1)]));
        assert!(hand.can_produce(&ManaCost([4, 0, 0, 0, 0, 0])));
        assert!(hand.can_produce(&ManaCost([1, 0, 0, 3, 0, 0])));        
        assert!(hand.can_produce(&ManaCost([0, 1, 0, 1, 0, 2])));
        assert!(!hand.can_produce(&ManaCost([1, 0, 0, 0, 1, 3])));
        assert!(!hand.can_produce(&ManaCost([0, 0, 0, 0, 0, 5])));
        assert!(!hand.can_produce(&ManaCost([1, 1, 1, 0, 0, 0])));
        let hand = Hand {
            cards: vec![
                &TAIGA,
                &TAIGA,
                &TAIGA,
            ],
        };
        assert!(hand.can_produce(&ManaCost([0, 0, 0, 1, 0, 0])));
        assert!(!hand.can_produce(&ManaCost([0, 0, 0, 3, 0, 0])));
        let hand = Hand {
            cards: vec![
                &MOX_R,
                &MOX_R,
                &MOX_R,
            ],
        };
        assert!(hand.can_produce(&ManaCost([0, 0, 0, 3, 0, 0])));
        let hand = Hand {
            cards: vec![
                &LOTUS,
                &MOX_G,
                &TAIGA,
            ],
        };
        println!("{}", hand);
        assert!(hand.can_produce(&ManaCost([0, 0, 0, 3, 2, 0])));
    }

    #[test]
    fn deck_tests() {
        let mut d = Deck::empty();
        d.add(&LOTUS, 1);
        d.add(&CITY, 4);
        d.add(&TAIGA, 4);
        let val = d.count_hands::<3>();
        assert!(val == ncr(9, 3));
        let val = d.count_hands_if::<1>(|h| h.can_produce(&ManaCost([3,0,0,0,0,0])));
        assert!(val == 1);
        let val = d.count_hands_if::<3>(|h| h.can_produce(&ManaCost([0,0,0,1,0,0])));
        dbg!(&val);
        dbg!(&ncr(9,3));
        assert!(val == ncr(9,3));
        let val = d.count_hands_if::<3>(|h| h.can_produce(&ManaCost([1,0,0,0,0,0])));
        assert!(val == ncr(9,3) - ncr(4,3));
        let mut d = Deck::empty();
        d.add(&LOTUS, 1);
        d.add(&CITY, 4);
        d.add(&TAIGA, 4);
        d.add(&BALL, 4);
        d.add(&BERSERK, 4);
        d.add(&BLOODLUST, 4);
        assert!(d.count_hands::<7>() == ncr(21, 7));
        dbg!(ncr(60, 7));
        assert!(ncr(60, 7) > 2);
    }
}
use ::std::collections::HashMap;

#[derive(Debug)]
enum TournamentFormat {
    SingleElemination,
    DoubleElemination,
}

#[derive(Debug)]
enum TournamentState {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug)]
enum MatchState {
    Scheduled,
    ongoing,
    Completed,
}

#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Match {
    match_id: u32,
    players: (u32, u32),
    scores: (u32, u32),
    state: MatchState,
}

#[derive(Debug)]
struct Round {
    round_number: u32,
    matches: Vec<Match>,
}

#[derive(Debug)]
struct PlayerStats {
    wins: u32,
    losses: u32,
    points_scored: u32,
}

#[derive(Debug)]
struct Tournament {
    id: u32,
    name: String,
    format: TournamentFormat,
    state: TournamentState,
    rounds: Vec<Round>,
    players: Vec<Player>,
    stats: HashMap<u32, PlayerStats>,
}

impl Tournament {
    fn new(name: &str, tournament_format: TournamentFormat, total_players: Vec<Player>) -> Self {
        let stat_player1 = PlayerStats {
            wins: 0,
            losses: 0,
            points_scored: 0,
        };

        let stat_player2 = PlayerStats {
            wins: 0,
            losses: 0,
            points_scored: 0,
        };

        let stat_player3 = PlayerStats {
            wins: 0,
            losses: 0,
            points_scored: 0,
        };

        let stat_player4 = PlayerStats {
            wins: 0,
            losses: 0,
            points_scored: 0,
        };

        let mut stats_map = HashMap::new();

        stats_map.insert(total_players[0].id, stat_player1);
        stats_map.insert(total_players[1].id, stat_player2);
        stats_map.insert(total_players[2].id, stat_player3);
        stats_map.insert(total_players[3].id, stat_player4);

        Self {
            id: 1,
            name: name.to_string(),
            format: tournament_format,
            state: TournamentState::NotStarted,
            rounds: vec![],
            players: total_players,
            stats: stats_map,
        }
    }

    fn start(&mut self) {
        match self.state {
            TournamentState::NotStarted => self.state = TournamentState::InProgress,
            TournamentState::Completed => println!("tournament already completed"),
            TournamentState::InProgress => println!("tournament already in progress"),
        }

        let players_12 = (self.players[0].id, self.players[1].id);
        let players_23 = (self.players[2].id, self.players[3].id);

        let match_1 = Match {
            match_id: 1,
            players: players_12,
            scores: (0, 0),
            state: MatchState::Scheduled,
        };

        let match_2 = Match {
            match_id: 2,
            players: players_23,
            scores: (0, 0),
            state: MatchState::Scheduled,
        };

        let round_1 = Round {
            round_number: 0,
            matches: vec![match_1, match_2],
        };

        self.rounds = vec![round_1];
    }
}

fn main() {
    let players = vec![
        Player {
            id: 1,
            name: "player1".to_string(),
        },
        Player {
            id: 2,
            name: "player2".to_string(),
        },
        Player {
            id: 3,
            name: "player3".to_string(),
        },
        Player {
            id: 4,
            name: "player4".to_string(),
        },
    ];

    let mut new_tournament = Tournament::new(
        "first tournament",
        TournamentFormat::SingleElemination,
        players,
    );

    println!("tournament information : {:#?}", new_tournament);
    new_tournament.start();
    println!("tournament information : {:#?}", new_tournament);
}

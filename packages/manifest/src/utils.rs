use crate::migrators::Migrator;

pub fn find_shortest_migration_path(
    migrators: Vec<Migrator>,
    from: &str,
    to: &str,
) -> Option<Vec<Migrator>> {
    if from == to {
        return Some(vec![]);
    };

    let possible_starts = migrators
        .iter()
        .filter(|node| node.from == from)
        .cloned()
        .collect::<Vec<Migrator>>();

    if possible_starts.is_empty() {
        return None;
    };

    let mut visited = possible_starts.clone();
    let mut queue = possible_starts
        .iter()
        .cloned()
        .map(|start| (start.clone(), vec![start]))
        .collect::<Vec<(Migrator, Vec<Migrator>)>>();

    for i in 0..queue.len() {
        let (node, path) = queue[i].clone();
        if node.to == to {
            return Some(path.clone());
        };

        let neighbors = migrators
            .iter()
            .filter(|neighbor| neighbor.from == node.to)
            .cloned()
            .collect::<Vec<Migrator>>();

        for neighbor in neighbors {
            visited.push(neighbor.clone());

            if neighbor.to == to {
                let mut path_clone = path.clone();
                path_clone.push(neighbor.clone());
                return Some(path_clone);
            };

            let mut path_clone = path.clone();
            path_clone.push(neighbor.clone());

            queue.push((neighbor, path_clone));
        }
    }

    None
}

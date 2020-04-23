// Copyright (c) The cargo-guppy Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Remove all dependency links that are dev-only.
//!
//! Dev-only dependencies are typically not included in release builds, so it's useful to be able
//! to filter out those links.

use guppy::graph::{DependencyDirection, PackageGraph, PackageLink};
use guppy::Error;
use std::iter;

fn main() -> Result<(), Error> {
    // `guppy` accepts `cargo metadata` JSON output. Use a pre-existing fixture for these examples.
    let fixture = include_str!("../fixtures/large/metadata_libra.json");
    let mut package_graph = PackageGraph::from_json(fixture)?;

    // Pick an important binary package and compute the number of dependencies.
    //
    // A clone is typically not required but in this case we're mutating the graph, so we need to
    // release the immutatable borrow.
    let libra_node_id = package_graph
        .workspace()
        .member_by_path("libra-node")
        .unwrap()
        .id()
        .clone();

    let before_count = package_graph
        .query_forward(iter::once(&libra_node_id))?
        .resolve()
        .into_ids(DependencyDirection::Forward)
        .count();
    println!("number of packages before: {}", before_count);

    let resolver_fn = |link: PackageLink<'_>| {
        if link.edge.dev_only() {
            println!(
                "*** filtering out dev-only link: {} -> {}",
                link.from.name(),
                link.to.name()
            );
            return false;
        }
        true
    };

    let query = package_graph.query_forward(iter::once(&libra_node_id))?;
    // Use `resolve_with` to filter out dev-only links.
    let resolve_with_len = query
        .clone()
        .resolve_with_fn(|_query, link| {
            // A package resolver allows for fine-grained control over which links are followed. In general,
            // it is anything that implements the `PackageResolver` trait.
            //
            // Functions with signature Fn(&PackageQuery<'_>, PackageLink<'_>) -> bool can be used
            // with `resolve_with_fn`.
            resolver_fn(link)
        })
        .into_ids(DependencyDirection::Forward)
        .len();
    println!("number of packages with resolve_with: {}", resolve_with_len);

    // Alternatively, `retain_edges` takes a closure that returns `true` if this edge should be kept in the graph.
    package_graph.retain_edges(|_data, link| {
        // '_data' contains metadata for every package. It isn't used in this example but some
        // complex filters may make use of it.
        resolver_fn(link)
    });

    // Iterate over all links and assert that there are no dev-only links.
    for link in package_graph
        .resolve_all()
        .into_links(DependencyDirection::Forward)
    {
        assert!(!link.edge.dev_only());
    }

    // Count the number of packages after.
    let after_count = package_graph
        .query_forward(iter::once(&libra_node_id))?
        .resolve()
        .into_ids(DependencyDirection::Forward)
        .count();
    println!("number of packages after retain_edges: {}", after_count);

    Ok(())
}

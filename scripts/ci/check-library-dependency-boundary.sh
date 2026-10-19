#!/usr/bin/env bash
# Shared companions: ci/cargo-dependency-boundaries.json
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

# This is Timers' library policy, not final-canister deployment admission. Cargo
# selects only the library; workspace probe feature unification must not turn a
# qualification dependency into an allowed production dependency.
CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 perl -MJSON::PP -e '
    use strict;
    use warnings;
    open my $policy, "<", "ci/cargo-dependency-boundaries.json" or die "$!\n";
    my $rules = decode_json(do { local $/; <$policy> });
    close $policy or die "$!\n";
    my %forbidden = map { $_->{dependency} => 1 } @$rules;
    delete @forbidden{qw(ic0 ic-cdk-executor)};

    open my $rustc, "-|", "rustc", "-vV" or die "cannot inspect native target: $!\n";
    my $identity = do { local $/; <$rustc> };
    close $rustc or die "rustc identity failed\n";
    my ($host) = $identity =~ /^host: ([A-Za-z0-9_-]+)$/m;
    defined($host) or die "missing rustc host target\n";
    for my $target ("wasm32-unknown-unknown", $host) {
        open my $cargo, "-|", "cargo", "tree", "--locked", "--offline",
            "-p", "ic-timers", "--all-features", "--target", $target,
            "--edges", "normal,build", "--prefix", "none", "--no-dedupe",
            "--format", "{p}|{f}" or die "cannot inspect library graph: $!\n";
        my $bytes = do { local $/; <$cargo> };
        close $cargo or die "library Cargo tree failed ($target)\n";
        my (%packages, %failures);
        for my $line (split /\n/, $bytes) {
            my ($package) = split /\|/, $line, 2;
            my ($name, $version, $features) = $line =~ /\A([A-Za-z0-9_-]+) v([^\s|]+)[^|]*\|(.*)\z/;
            defined($name) or die "invalid library Cargo tree record: $line\n";
            $packages{$name}{$package} = 1;
            $failures{"forbidden production dependency: $name"} = 1
                if $forbidden{$name}
                || $name =~ /\Aic-(?:build(?:-|\z)|host-|testkit(?:-|\z)|wire(?:-|\z)|timers-(?:pocketic|runtime-probe|size-probe|test-protocol)\z)/;
            if ($name eq "ic-platform") {
                $failures{"forbidden Platform feature: $_"} = 1
                    for grep { $_ ne "executor" && $_ ne "ic0" } split /,/, $features;
            }
        }
        # Changes to these upstream identities require renewed combined runtime
        # qualification, not automatic acceptance of another resolved generation.
        for my $expected (["ic-cdk-executor", "2.0.0"], ["ic0", "1.2.0"]) {
            my ($name, $version) = @$expected;
            my @found = sort keys %{$packages{$name} // {}};
            $failures{"expected one $name $version; found: @found"} = 1
                unless @found == 1 && $found[0] eq "$name v$version";
        }
        exists($packages{"ic-timers"}) && exists($packages{"ic-platform"})
            or die "library graph is missing its root or Platform\n";
        die "Library dependency boundary failed ($target):\n" . join("\n", sort keys %failures) . "\n"
            if %failures;
        print "Library dependency boundary passed ($target, all features, normal/build).\n";
    }
'

#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
perl <<'PERL'
use strict;
use warnings;
use File::Find;
my $root = 'crates/ic-timers/src';
open my $facade, '<', "$root/lib.rs" or die "$root/lib.rs: $!";
my $source = do { local $/; <$facade> };
$source =~ /^mod platform;$/m or die "platform must remain a private module\n";
$source =~ /^#!\[forbid\(private_interfaces\)\]$/m
    or die "private types must not leak into public interfaces\n";
$source =~ /^#!\[deny\(unsafe_code\)\]$/m
    or die "unsafe code must be denied by default\n";
my $ffi = "$root/platform/ffi.rs";
-f $ffi or die "missing FFI boundary\n";
find({ no_chdir => 1, wanted => sub {
    return unless /\.rs$/ && -f $_;
    my $file = $_;
    open my $input, '<', $file or die "$file: $!";
    my $text = do { local $/; <$input> };
    if ($file =~ m{/platform/} && $text =~ /\bpub\s+(?:(?:async|unsafe)\s+)*(?:struct|enum|type|fn|use|mod|trait|const|static|extern)\b/) {
        die "platform items require restricted visibility: $file\n";
    }
    if ($file ne $ffi && $text =~ /(?:\bunsafe\s*(?:\{|extern|fn)|unsafe\(export_name|allow\(unsafe_code\)|ic0::call_(?:new|on_cleanup)\b)/) {
        die "unsafe operations and IC callback registration belong to FFI: $file\n";
    }
    if ($file !~ m{/platform/} && $text =~ /ic0::global_timer_set|ic_cdk_executor::/) {
        die "timer execution belongs to the private platform: $file\n";
    }
}}, $root);
PERL
echo "Provider boundary checks passed"

use dioxus::prelude::*;
use rand::seq::SliceRandom;

const TERRORS: [&str; 69] = [
    "Automation Anxieties - 'How does this script work again?'",
    "Backup Bedlam - Is anyone sure the backups work?",
    "CDK Conundrums - Who can construct the cloud without you?",
    "Database Drama - 'Who dropped the database?'",
    "Succession Scares - Who will be the new you?",
    "Tooling Terrors - 'We had a person for that?'",
    "Update Uproar - 'We were supposed to update WHAT every month?'",
    // AWS
    "Cloud Cost Creep - The bill is how much?!",
    "CloudFormation Frustrations - Templates that fail with cryptic errors.",
    "CloudWatch Confusions - Alerts set for every minute of every day.",
    "Cost Calculator Confusion - Budgets blown by misunderstood metrics.",
    "Cost Management Mysteries - Unforeseen charges popping up like whack-a-mole.",
    "Documentation Despair - 'Wait, you actually read that?'",
    "EC2 Enigmas - Instance types more confusing than a calculus exam.",
    "EC2 Enigmas - Instances that start behaving like they're haunted.",
    "Elastic Load Balancer Blues - Balancing more than just traffic.",
    "Elastic Load Balancing Blunders - One does not simply load balance.",
    "IAM Ineptitude - 'Who has access to what again?'",
    "Lambda Labyrinths - 'Wait, which handler does what?'",
    "Lambda Limits - 'You mean there's a timeout?'",
    "Monitoring Mysteries - Alerts go off. Does anyone know why?",
    "Networking Nightmares - 'You mean there's more than one subnet?'",
    "RDS Riddles - Multi-AZ, Read Replicas, and Snapshots, oh my!",
    "Region Regrets - 'Did we just deploy to the wrong AWS region?'",
    "Route 53 Regrets - DNS changes that take longer than expected.",
    "S3 Misconfigurations - Public buckets becoming public nightmares.",
    "S3 Surprises - Public buckets making unexpected headlines.",
    "Scalability Scares - 'It scales, right?'",
    "Secrets Mismanagement - Not everyone knows what should be secret.",
    "Service Sprawl - Too many services, too little understanding.",
    "The AWS Abyss - Who knows which button to press now?",
    "VPC Vexations - Subnet math is harder than actual math.",
    // Docker
    "Build Breaks - \"It says 'Layer not found'.\"",
    "Container Crashes - Containers that terminate for ghost reasons.",
    "Dependency Debacles - 'I thought we were using the same base image?'",
    "Docker Dilemmas - 'But it worked on my machine!'",
    "Image Inflation - 'Why is this image 10GB?'",
    "Logging Lapses - Logs that vanish when containers do.",
    "Networking Nuisances - Inter-container communication gone wrong.",
    "Orchestration Overload - Too many containers, not enough oversight.",
    "Registry Riddles - Pushes and pulls that perplex the unprepared.",
    "Security Slip-ups - Unpatched images leading to vulnerabilities.",
    "Volume Voodoo - Persistent data disappearing into thin air.",
    // Kubernetes
    "Cluster Conflicts - Clashing configurations across clusters.",
    "ConfigMap Conundrums - Mysterious misconfigurations.",
    "Configuration Conflicts - YAML files, YAML problems.",
    "Helm Hell - Charts that chart a course to chaos.",
    "Ingress Insanity - Routing rules that make no sense.",
    "Kubernetes Chaos - Pods and nodes everywhere but nowhere.",
    "Node Nuisances - Nodes not playing nice with scheduling.",
    "Pod Puzzles - Pods in perpetual pending.",
    "RBAC Riddles - Access controls that control too much or too little.",
    "Resource Quotas Quirks - Pods that can't pod because of quotas.",
    "Secrets Spillage - Secrets not as secret as thought.",
    "StatefulSet Sorrows - Persistent storage proves persistently problematic.",
    // CI/CD
    "Artifact Anxieties - 'Where did the artifact go?'",
    "CI/CD Clogs - Pipelines more clogged than city traffic.",
    "Deployment Doldrums - Deployments that deploy despairs.",
    "Environment Entanglements - Dev, staging, and prod look eerily similar.",
    "Pipeline Paralysis - Builds that never build.",
    "Rollback Riddles - 'How do we rollback to a working state?'",
    "Scripting Scares - Script",
    // Git
    "Branch Bungles - 'I thought we were on the same branch?'",
    "Commit Confusion - 'What does this commit even do?'",
    "Merge Madness - 'Why is this merge taking so long?'",
    "Rebase Regrets - 'I should have just merged.'",
    "Conventional Commit Confusion - What does fix: mean?.",
    "Git Gripes - 'I thought 'force push' was a Star Wars move?'",
    "Git Nightmares - 'Why is the history so messy?'",
];

#[component]
pub fn Terror() -> Element {
    let mut rng = rand::thread_rng();
    let terror = TERRORS.choose(&mut rng).unwrap();

    rsx! {
        h1 { class: "text-xl sm:text-2xl md:text-4xl font-bold mb-3 sm:mb-4 md:mb-6",
            "{terror}"
        }
    }
}

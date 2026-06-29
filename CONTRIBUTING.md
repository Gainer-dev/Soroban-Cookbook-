# Contributing to Soroban Cookbook

Welcome! 👋 We're thrilled you're interested in contributing to the Soroban Cookbook. This guide will help you get started and make your first contribution.

## Table of Contents

- [Welcome](#welcome)
- [Getting Started](#getting-started)
- [Where to Ask Questions](#where-to-ask-questions)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Style Guide](#style-guide)
- [Resources](#resources)

## Welcome

The Soroban Cookbook is a community-driven collection of recipes, examples, and best practices for building on the Soroban smart contract platform. Whether you're a seasoned blockchain developer or just starting out, your contributions are valuable.

### Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please be respectful, inclusive, and constructive in all interactions.

## Getting Started

### Prerequisites

- **Git**: Version control system ([Install Git](https://git-scm.com/downloads))
- **GitHub Account**: Required for contributing ([Create Account](https://github.com/join))
- **Basic Markdown**: Our documentation uses Markdown ([Learn Markdown](https://www.markdownguide.org/getting-started/))
- **Rust/Soroban Knowledge** (optional): Helpful but not required for documentation contributions

### First Steps

1. **Fork the Repository**
   - Visit [Soroban-Cookbook/Soroban-Cookbook](https://github.com/Soroban-Cookbook/Soroban-Cookbook)
   - Click the "Fork" button in the top-right corner

2. **Clone Your Fork**
   ```bash
   git clone https://github.com/YOUR-USERNAME/Soroban-Cookbook.git
   cd Soroban-Cookbook
   ```

3. **Set Up Upstream Remote**
   ```bash
   git remote add upstream https://github.com/Soroban-Cookbook/Soroban-Cookbook.git
   ```

4. **Create a Branch**
   ```bash
   git checkout -b your-feature-branch
   ```

5. **Make Your Changes**
   - Edit existing files or create new ones
   - Follow our [Style Guide](#style-guide)

6. **Commit and Push**
   ```bash
   git add .
   git commit -m "your descriptive commit message"
   git push origin your-feature-branch
   ```

7. **Create a Pull Request**
   - Go to your fork on GitHub
   - Click "Pull Request" and follow the template

## Where to Ask Questions

### Community Channels

| Channel | Purpose | Link |
|---------|---------|------|
| **Discord** | Real-time chat, quick questions | [Join Discord](https://discord.gg/soroban) |
| **GitHub Discussions** | Longer discussions, ideas | [Start Discussion](https://github.com/Soroban-Cookbook/Soroban-Cookbook/discussions) |
| **GitHub Issues** | Bug reports, feature requests | [Open Issue](https://github.com/Soroban-Cookbook/Soroban-Cookbook/issues) |
| **Stellar Community** | General Stellar/Soroban help | [Stellar Community](https://community.stellar.org/) |

### Best Practices for Asking Questions

1. **Search first**: Check if your question has already been answered
2. **Be specific**: Include relevant code, error messages, and what you've tried
3. **Use the right channel**: Quick questions → Discord, Detailed discussions → GitHub Discussions
4. **Be patient**: Community members volunteer their time

## How to Contribute

### Types of Contributions

#### 1. 🐛 Reporting Bugs

Open a [GitHub Issue](https://github.com/Soroban-Cookbook/Soroban-Cookbook/issues/new) with:
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Screenshots if applicable

#### 2. 💡 Suggesting Features

Open a [GitHub Issue](https://github.com/Soroban-Cookbook/Soroban-Cookbook/issues/new) with:
- Use case and motivation
- Proposed solution
- Alternatives considered
- Example usage (if applicable)

#### 3. 📝 Documentation

- Fix typos or clarify existing content
- Add new recipes or examples
- Improve code comments
- Translate content to other languages

#### 4. 💻 Code Contributions

- Implement new features
- Fix bugs
- Add tests
- Improve performance

#### 5. 🧪 Testing

- Write unit tests
- Perform integration testing
- Report test failures
- Improve test coverage

### Contribution Workflow

1. **Find an Issue**: Look for issues labeled `good first issue` or `help wanted`
2. **Comment**: Let others know you're working on it
3. **Discuss**: For complex changes, discuss approach first
4. **Implement**: Write your code following our standards
5. **Test**: Ensure all tests pass
6. **Review**: Address reviewer feedback
7. **Merge**: Your contribution is merged! 🎉

## Development Workflow

### Branch Naming Convention

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
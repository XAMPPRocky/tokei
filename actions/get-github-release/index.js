// GitHub Actions script.
const actions = require('@actions/core')

const { getGitHubRelease } = require('./lib.js')

// Configuration
const repo = actions.getInput('repo', { required: true })
const installPath = actions.getInput('install_path') || undefined
const owner = actions.getInput('owner', { required: true })
const matches = new RegExp(actions.getInput('matches', { required: true }))
const token = actions.getInput('token', { required: true })

getGitHubRelease(owner, repo, matches, token, installPath)

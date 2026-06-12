/**
 * Phase 2 catalog fixture — mirrors fan-edits get.jsx (subset).
 */
import React, { useState, useEffect } from 'react'
import { MarkdownRenderer } from '@clevertree/markdown'

function findProject(projects, slug) {
  for (let i = 0; i < projects.length; i++) {
    if (projects[i].slug === slug) return projects[i]
  }
  return null
}

function countCategory(projects, category) {
  let n = 0
  for (let i = 0; i < projects.length; i++) {
    if (projects[i].category === category) n++
  }
  return n
}

export default function Catalog() {
  const [projects, setProjects] = useState([])
  const [view, setView] = useState('catalog')
  const [selectedSlug, setSelectedSlug] = useState('')
  const [readme, setReadme] = useState('')
  const [readmeError, setReadmeError] = useState('')

  useEffect(() => {
    const res = fetch('/index.json')
    if (res && res.ok === true && res.text) {
      try {
        const data = JSON.parse(res.text)
        if (data && data.projects) setProjects(data.projects)
      } catch (e) {}
    }
  }, [])

  useEffect(() => {
    if (view !== 'project' || !selectedSlug) return
    const project = findProject(projects, selectedSlug)
    if (!project) {
      setReadmeError('Project not found')
      return
    }
    const res = fetch('/' + project.path + '/README.md')
    if (res && res.ok === true && res.text) {
      setReadme(res.text)
      setReadmeError('')
    } else {
      setReadmeError('Could not load README')
      setReadme('')
    }
  }, [view, selectedSlug, projects])

  if (view === 'catalog') {
    return (
      <div className="catalog-root">
        <h1>Fan Edits</h1>
        <p className="catalog-lead">Projects from index.json</p>
        <div className="catalog-sections">
          <button className="nav-btn" onClick={() => setView('fanmixes')}>FanMixes ({countCategory(projects, 'FanMixes')})</button>
          <button className="nav-btn" onClick={() => setView('kidfriendly')}>KidFriendly ({countCategory(projects, 'KidFriendly')})</button>
        </div>
      </div>
    )
  }

  if (view === 'fanmixes' || view === 'kidfriendly') {
    const cat = view === 'fanmixes' ? 'FanMixes' : 'KidFriendly'
    const items = []
    for (let i = 0; i < projects.length; i++) {
      if (projects[i].category === cat) items.push(projects[i])
    }
    return (
      <div className="catalog-root">
        <button className="nav-btn" onClick={() => setView('catalog')}>Back</button>
        <h2>{cat}</h2>
        <ul className="project-list">
          {items.map(p => (
            <li key={p.slug}>
              <button className="project-btn" onClick={() => { setSelectedSlug(p.slug); setView('project') }}>{p.title}</button>
            </li>
          ))}
        </ul>
      </div>
    )
  }

  const project = findProject(projects, selectedSlug)
  const body = readmeError ? '# Error\n\n' + readmeError : (readme || '# Loading…')
  return (
    <div className="catalog-root project-view">
      <button className="nav-btn" onClick={() => setView('fanmixes')}>Back</button>
      <h2>{project ? project.title : selectedSlug}</h2>
      <div className="readme-panel">
        <MarkdownRenderer content={body} />
      </div>
    </div>
  )
}

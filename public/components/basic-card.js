import { render, html } from '//cdn.pika.dev/lighterhtml';

const basicCard = siteData => html`
  <div class="basic-card">
    <div class="basic-card-image" hidden=${!siteData.image}>
      <img async src=${siteData.image}>
    </div>
    <div class="basic-card-hr" hidden=${!siteData.image}></div>
    <div class="basic-card-header" hidden=${!siteData.title && !siteData.name}>
      <div class="basic-card-title" hidden=${!siteData.title}>${siteData.title}</div>
      <div class="basic-card-subtitle" hidden=${!siteData.name}>${siteData.name}</div>
    </div>
    <div class="basic-card-content" hidden=${siteData.description}>
      <div class="basic-card-description" hidden=${!siteData.description}>${siteData.description}</div>
    </div>
    <div class="basic-card-hr" hidden=${!!siteData.image}></div>
  </div>
`

export default basicCard
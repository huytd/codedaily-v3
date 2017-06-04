import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

class App extends React.Component {
  fetch_links() {
    let that = this;

    fetch("/api/feed")
      .then(response => response.json())
      .then(json => {
        that.setState({
          links: Array.from(json.message) || []
        });
        console.log(json);
      });
  }

  decodeEntities(encodedString) {
    var textArea = document.createElement('textarea');
    textArea.innerHTML = encodedString;
    return textArea.value;
  }

  constructor(props) {
    super(props);
    this.state = {
      links: []
    };
    this.fetch_links();
  }
  
  render() {
    let list = this.state.links.map(link => {
      let date = new Date(link.time * 1000);
      return <li key={link.time}>
        <a href={link.url} target="_blank" rel="nofollow">
          <div className="post-title">{this.decodeEntities(link.title)}</div>
          <div className="post-meta">Đăng ngày <span>{date.toLocaleDateString()}</span> tại <span>{link.source}</span></div>
        </a>
      </li>
    });

    return (
      <div className="container">
        <div className="header">
          <span>c</span> codedaily
        </div>
        <div className="content">
          <div className="main">
            <ul className="link-list">
              { list }
            </ul>
          </div>
        </div>
      </div>
    );
  }
}

export default App;

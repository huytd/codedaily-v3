import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

const LINKS_PER_PAGE = 30;

class App extends React.Component {
  fetch_links(page) {
    let that = this;

    fetch(`/api/feed/${page || 1}`)
      .then(response => response.json())
      .then(json => {
        that.setState({
          links: Array.from(json.links) || [],
          total: json.total,
          currentPage: page || 1
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
      links: [],
      total: 0,
      currentPage: 1
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

    let totalPage = parseInt(this.state.total / LINKS_PER_PAGE);
    let pages = Array(totalPage).fill(0).map((page, idx) => {
      let id = idx + 1;
      return <li onClick={this.fetch_links.bind(this, id)} className={this.state.currentPage == id ? 'active' : ''} key={`page-${id}`}>
        { id }
      </li>
    });

    return (
      <div className="container">
        <div className="header">
          <span>k</span> kipalog links
          <div className="user-control">
            <ul className="filter-list">
              <li><a href="#">Top</a></li>
              <li><a className="active" href="#">Latest</a></li>
              <li><a href="#">Login</a></li>
            </ul>
          </div>
        </div>
        <div className="content">
          <div className="main">
            <ul className="link-list">
              { list }
            </ul>
            <ul className="pagination">
              { pages }
            </ul>
          </div>
        </div>
      </div>
    );
  }
}

export default App;

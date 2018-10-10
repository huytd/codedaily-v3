import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';
import Login from './login';
import Register from './register';

import utils from '../services/util';
import AuthenticationService from '../services/authentication-service';
import LinkService from '../services/link-service';

const LINKS_PER_PAGE = 30;
class App extends React.Component {
  fetch_links(page) {
    let that = this;
    LinkService.getLinks(page)
    .then(result => {
      const {links, total, currentPage} = result;
      that.setState({links, total, currentPage});
      window.scrollTo(0, 0);
    });
  }

  constructor(props) {
    super(props);
    this.state = {
      links: [],
      total: 0,
      currentPage: 1,
      isLoggedIn: false,
      loggedInUser: null,
      showLogin: false,
      showRegister: false,
      menuActive: false
    };
  }

  componentDidMount() {
    this.fetch_links();
    this.checkForLogin();
  }

  toggleMenu() {
    let menuState = !this.state.menuActive;
    this.setState({
      menuActive: menuState
    });
  }

  checkForLogin() {
    const {isLoggedIn, loggedInUser} = AuthenticationService.checkForLogin();
    this.setState({isLoggedIn, loggedInUser});
  }

  showLoginComponent() {
    if (this.state.isLoggedIn) {
      return "";
    } else {
      if (this.state.showLogin) {
        return (<Login/>);
      }
      return "";
    }
  }

  showRegisterComponent() {
    if (this.state.showRegister) {
      return (<Register/>);
    }
    return "";
  }

  displayRegister() {
    this.toggleMenu();
    this.setState({
      showRegister: true
    });
  }

  displayLogin() {
    this.toggleMenu();
    this.setState({
      showLogin: true
    });
  }

  doLogout() {
    AuthenticationService.doLogout();
    utils.reloadAfter(100);
    this.toggleMenu();
  }

  showLoggedInUser() {
    if (this.state.isLoggedIn) {
      return [
        (<li key="user-name"><a onClick={this.toggleMenu.bind(this)}><b>{this.state.loggedInUser}</b></a></li>),
        (<li key="logout-button"><a onClick={this.doLogout}>Logout</a></li>)
      ];
    } else {
      return [
        (<li key="login-button"><a onClick={this.displayLogin.bind(this)}>Login</a></li>),
        (<li key="register-button"><a onClick={this.displayRegister.bind(this)}>Register</a></li>)
      ];
    }
  }

  filterText(input){
      return input.replace(/^\"/, '').replace(/\"$/, '').replace(/\\\"/g, '"')
          .replace(/(&#8220|&#8221|&#8230)/g, "").replace(/;/g, "")
          .replace(/<\/?(p|ul|li|b|i|span|div|tr|td|table|em|strong|hr|h1|h2|h3|h4|h5|h6)>/g, '')
          .replace(/<a.*>/g, '').replace(/<\/a>/g, '');
  }

  render() {
    let maxWords = 60;
    let list = this.state.links.map((link, idx) => {
        let desc = this.filterText(link.body);
            desc = desc.split(' ').slice(0, maxWords).join(" ").trim();
        desc += (desc.length > 0 && desc[desc.length-1] !== '.') ? '...' : '';
        let date = new Date(link.time * 1000);
        return <li key={link.time + "-" + idx}>
            <a href={link.url} target="_blank" rel="nofollow">
            <div className="post-title">{utils.decodeEntities(link.title)}</div>
            </a>
            <div className="post-desc">{desc}</div>
            <div className="post-meta">Đăng ngày <span>{date.toLocaleDateString()}</span> tại <span>{link.source}</span></div>
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
            <button className="toggle-icon" onClick={this.toggleMenu.bind(this)}></button>
            <ul className={"filter-list" + (this.state.menuActive ? " show": " hide")}>
              <li><a href="#" onClick={this.toggleMenu.bind(this)}>Top</a></li>
              <li><a className="active" href="#" onClick={this.toggleMenu.bind(this)}>Latest</a></li>
              { this.showLoggedInUser() }
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

import 'whatwg-fetch';

class LinkService {
  getLinks(page = 1) {
    return fetch(`/api/feed/${page}`)
      .then(response => response.json())
      .then(json => {
        if (!json.links || json.links === null) json.links = []; 
        return {
          links: Array.from(json.links) || [],
          total: json.total,
          currentPage: page
        };
      });
  }
}

module.exports = new LinkService();

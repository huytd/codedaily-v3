import LinkService from '../../services/link-service';
import fetchMock from 'fetch-mock';

describe('Link Service', () => {
  afterEach(() => {
    fetchMock.restore();
  });

  it('Should get links from feed api', async () => {
    const page = 1;

    fetchMock.get(`/api/feed/${page}`, {
      links: [{ link: 'dummy'}],
      total: 10
    });

    expect.assertions(3);
    const links = await LinkService.getLinks(page);
    expect(links.links.length).toEqual(1);
    expect(links.total).toEqual(10);
    expect(links.currentPage).toEqual(page);
  });

  it('Should return empty array if do not get any link', async () => {
    fetchMock.get('/api/feed/1', {
      total: 10
    });

    expect.assertions(1);
    const links = await LinkService.getLinks(1);
    console.log(links.links);
    expect(links.links.length).toEqual(0);
  });
});

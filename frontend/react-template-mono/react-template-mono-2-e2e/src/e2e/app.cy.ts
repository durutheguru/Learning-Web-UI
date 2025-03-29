import { getGreeting } from '../support/app.po';

describe('react-template-mono-2-react-template-mono-2', () => {
  beforeEach(() => cy.visit('/'));

  it('should display welcome message', () => {
    // Custom command example, see `../support/commands.ts` file
    cy.login('my-email@something.com', 'myPassword');

    // Function helper example, see `../support/app.po.ts` file
    getGreeting().contains(
      'Welcome react-template-mono-2-react-template-mono-2'
    );
  });
});

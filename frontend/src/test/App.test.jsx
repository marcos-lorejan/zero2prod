import { render, screen, fireEvent } from '@testing-library/react';
import App from '../App';

test('renders sign up form', () => {
  render(<App />);
  const heading = screen.getByRole('heading', { name: /Sign Up/i });
  expect(heading).toBeInTheDocument();
});

test('submits form with email', async () => {
  const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
  global.fetch = vi.fn(() =>
    Promise.resolve({
      text: () => Promise.resolve('Success!'),
    })
  );

  render(<App />);
  const input = screen.getByPlaceholderText(/Enter your email/i);
  const button = screen.getByRole('button', { name: /Sign Up/i });

  fireEvent.change(input, { target: { value: 'test@example.com' } });
  fireEvent.click(button);

  await new Promise(resolve => setTimeout(resolve, 100));
  expect(fetch).toHaveBeenCalledWith('http://localhost:3000/signup', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email: 'test@example.com' }),
  });
  expect(alertMock).toHaveBeenCalledWith('Success!');
});

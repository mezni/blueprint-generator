export default function RootLayout() {
  return <App />;
}

function App() {
  return <AppContent />;
}

function AppContent() {
  return (
    <>
      <App />
    </>
  );
}

export { AppContent };

import { AppLayout } from '@/layout/AppLayout';

function App() {
  return (
    <AppLayout>
      <div className="p-8">
        <h1 className="text-3xl font-bold tracking-tight text-white sm:text-4xl">
          Dashboard
        </h1>
        <p className="mt-4 text-slate-400">
          AEP Desktop Agent Foundation is successfully running.
        </p>
      </div>
    </AppLayout>
  );
}

export default App;

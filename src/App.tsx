import { AppLayout } from '@/layout/AppLayout';
import { ScanDashboard } from '@/components/ScanDashboard';

function App() {
  return (
    <AppLayout>
      <div className="p-8 w-full">
        <ScanDashboard />
      </div>
    </AppLayout>
  );
}

export default App;

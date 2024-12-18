import { useState } from "react";
import { Navbar } from "~/components/Navbar";
import { RefuelForm } from "~/components/RefuelForm";
import { LogoutModal } from "~/components/Modals/LogoutModal";

export default function RefuelPage() {
  const [isLogoutModalOpen, setIsLogoutModalOpen] = useState(false);

  // Dummy vehicle data
  const vehicles = [
    { id: "1", brand: "Toyota", model: "Camry" },
    { id: "2", brand: "Honda", model: "Civic" },
  ];

  const handleLogout = async () => {
    // Replace with actual logout API call
    console.log("Logged out");
    setIsLogoutModalOpen(false);
    window.location.href = "/";
  };

  return (
    <>
      <Navbar onLogout={() => setIsLogoutModalOpen(true)} />
      <div className="container mx-auto p-4">
        <h1 className="text-2xl font-bold mb-4">Refuel Vehicle</h1>
        <RefuelForm vehicles={vehicles} onSubmit={(data) => console.log("Submitted:", data)} />
      </div>
      <LogoutModal isOpen={isLogoutModalOpen} onClose={() => setIsLogoutModalOpen(false)} onConfirm={handleLogout} />
    </>
  );
}

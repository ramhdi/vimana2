import { Link, useNavigate } from "@remix-run/react";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "~/components/ui/dialog";

export function Navbar() {
  const [isLogoutModalOpen, setIsLogoutModalOpen] = useState(false);
  const navigate = useNavigate();

  const handleLogout = async () => {
    try {
      const response = await fetch("/api/protected/logout", {
        method: "POST",
        credentials: "same-origin",
      });

      if (response.ok) {
        navigate("/");
      } else {
        alert("Failed to log out. Please try again.");
      }
    } catch (error) {
      alert("An error occurred while logging out.");
    } finally {
      setIsLogoutModalOpen(false);
    }
  };

  return (
    <>
      <nav className="bg-white shadow-md sticky top-0 z-10">
        <div className="container mx-auto flex justify-between items-center p-4">
          <Link to="/home" className="text-2xl font-bold text-gray-800">
            Vimana
          </Link>
          <div className="flex items-center space-x-4">
            <Link
              to="/home"
              className="text-gray-600 hover:text-gray-800 transition-colors"
            >
              Vehicles
            </Link>
            <Link
              to="#"
              className="text-gray-600 hover:text-gray-800 transition-colors"
            >
              Maintenances
            </Link>
            <Link
              to="/refuel"
              className="text-gray-600 hover:text-gray-800 transition-colors"
            >
              Refuel
            </Link>
            <Button
              variant="destructive"
              onClick={() => setIsLogoutModalOpen(true)}
            >
              Logout
            </Button>
          </div>
        </div>
      </nav>

      {/* Logout Confirmation Modal */}
      <Dialog open={isLogoutModalOpen} onOpenChange={setIsLogoutModalOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirm Logout</DialogTitle>
          </DialogHeader>
          <div className="mt-4">
            <p className="text-sm text-gray-600">
              Are you sure you want to log out? You will need to log back in to access your account.
            </p>
          </div>
          <div className="flex justify-end space-x-4 mt-6">
            <Button variant="secondary" onClick={() => setIsLogoutModalOpen(false)}>
              Cancel
            </Button>
            <Button variant="destructive" onClick={handleLogout}>
              Logout
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}

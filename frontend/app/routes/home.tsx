import { useEffect, useState } from "react";
import { Navbar } from "~/components/Navbar";
import { VehicleCard } from "~/components/VehicleCard";
import { DeleteVehicleModal } from "~/components/Modals/DeleteVehicleModal";
import { Button } from "~/components/ui/button";
import { Vehicle } from "~/types/vehicle";
import { vehicleService } from "~/services/vehicleService";
import { VehicleModal } from "~/components/Modals/VehicleModal";

export default function Dashboard() {
  const [vehicles, setVehicles] = useState<Vehicle[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isVehicleModalOpen, setIsVehicleModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<"create" | "edit">("create");
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [selectedVehicle, setSelectedVehicle] = useState<Vehicle | null>(null);

  useEffect(() => {
    const loadVehicles = async () => {
      setIsLoading(true);
      try {
        const data = await vehicleService.fetchVehicles();
        setVehicles(data);
      } catch (error) {
        alert("Failed to load vehicles");
      } finally {
        setIsLoading(false);
      }
    };

    loadVehicles();
  }, []);

  const handleAddVehicle = async (vehicle: Omit<Vehicle, "id">) => {
    try {
      const newVehicle = await vehicleService.createVehicle(vehicle);
      setVehicles([...vehicles, newVehicle]);
      setIsVehicleModalOpen(false);
    } catch {
      alert("Failed to add vehicle");
    }
  };

  const handleEditVehicle = async (updatedVehicle: Vehicle) => {
    try {
      await vehicleService.updateVehicle(updatedVehicle);
      setVehicles(
        vehicles.map((v) => (v.id === updatedVehicle.id ? updatedVehicle : v))
      );
      setIsVehicleModalOpen(false);
    } catch {
      alert("Failed to update vehicle");
    }
  };

  const handleDeleteVehicle = async () => {
    if (!selectedVehicle) return;

    try {
      await vehicleService.deleteVehicle(selectedVehicle.id);
      setVehicles(vehicles.filter((v) => v.id !== selectedVehicle.id));
      setIsDeleteModalOpen(false);
    } catch {
      alert("Failed to delete vehicle");
    }
  };

  return (
    <div>
      <Navbar />
      <div className="container mx-auto p-4">
        <div className="flex justify-between items-center">
          <h2 className="text-2xl font-bold text-gray-800">Your Vehicles</h2>
          <Button
            onClick={() => {
              setModalMode("create");
              setSelectedVehicle(null);
              setIsVehicleModalOpen(true);
            }}
          >
            Add Vehicle
          </Button>
        </div>

        {isLoading ? (
          <p className="text-center text-gray-500 mt-6">Loading vehicles...</p>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 mt-6">
            {vehicles.map((vehicle) => (
              <VehicleCard
                key={vehicle.id}
                {...vehicle}
                onEdit={() => {
                  setModalMode("edit");
                  setSelectedVehicle(vehicle);
                  setIsVehicleModalOpen(true);
                }}
                onDelete={() => {
                  setSelectedVehicle(vehicle);
                  setIsDeleteModalOpen(true);
                }}
              />
            ))}
          </div>
        )}
      </div>

      {/* Modals */}
      <VehicleModal
        isOpen={isVehicleModalOpen}
        onClose={() => setIsVehicleModalOpen(false)}
        onSubmit={modalMode === "create" ? handleAddVehicle : handleEditVehicle}
        vehicle={modalMode === "edit" ? selectedVehicle : undefined}
        mode={modalMode}
      />
      <DeleteVehicleModal
        isOpen={isDeleteModalOpen}
        onClose={() => setIsDeleteModalOpen(false)}
        onConfirm={handleDeleteVehicle}
        vehicle={selectedVehicle}
      />
    </div>
  );
}

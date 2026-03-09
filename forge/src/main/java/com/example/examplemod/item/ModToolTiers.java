package com.example.examplemod.item;

import com.example.examplemod.MyMod;
import com.example.examplemod.util.ModTags;
import java.util.List;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.world.item.Tier;
import net.minecraft.world.item.Tiers;
import net.minecraft.world.item.crafting.Ingredient;
import net.minecraftforge.common.ForgeTier;
import net.minecraftforge.common.TierSortingRegistry;

public class ModToolTiers {

  public static final Tier SAPPHIRE =
      TierSortingRegistry.registerTier(
          new ForgeTier(
              5,
              2000,
              10,
              4,
              20,
              ModTags.Blocks.NEEDS_SAPPHIRE_TOOL,
              () -> Ingredient.of(ModItems.SAPPHIRE.get())),
          ResourceLocation.fromNamespaceAndPath(MyMod.MODID, "sapphire"),
          List.of(Tiers.NETHERITE),
          List.of());
}

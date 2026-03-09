package com.example.examplemod.item;

import com.example.examplemod.MyMod;
import com.example.examplemod.block.ModBlocks;
import com.example.examplemod.entity.ModEntities;
import com.example.examplemod.item.custom.FuelItem;
import com.example.examplemod.item.custom.MetalDetectorItem;
import com.example.examplemod.item.custom.ModArmorItem;
import com.example.examplemod.sound.ModSounds;
import net.minecraft.world.item.ArmorItem;
import net.minecraft.world.item.AxeItem;
import net.minecraft.world.item.HoeItem;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemNameBlockItem;
import net.minecraft.world.item.PickaxeItem;
import net.minecraft.world.item.RecordItem;
import net.minecraft.world.item.ShovelItem;
import net.minecraft.world.item.SwordItem;
import net.minecraftforge.common.ForgeSpawnEggItem;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.registries.DeferredRegister;
import net.minecraftforge.registries.ForgeRegistries;
import net.minecraftforge.registries.RegistryObject;

public class ModItems {
  public static final DeferredRegister<Item> ITEMS =
      DeferredRegister.create(ForgeRegistries.ITEMS, MyMod.MODID);

  public static final RegistryObject<Item> RHINO_SPANW_EGG =
      ITEMS.register(
          "rhino_spanw_egg",
          () ->
              new ForgeSpawnEggItem(ModEntities.RHINO, 0x7e67a6, 0xffffff, new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE =
      ITEMS.register("sapphire", () -> new Item(new Item.Properties()));

  public static final RegistryObject<Item> RAW_SAPPHIRE =
      ITEMS.register("raw_sapphire", () -> new Item(new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE_STAFF =
      ITEMS.register("sapphire_staff", () -> new Item(new Item.Properties().stacksTo(1)));

  public static final RegistryObject<Item> METAL_DETECTOR =
      ITEMS.register(
          "metal_detector", () -> new MetalDetectorItem(new Item.Properties().durability(100)));

  public static final RegistryObject<Item> STRAWBERRY =
      ITEMS.register("strawberry", () -> new Item(new Item.Properties().food(ModFoods.STRAWBERRY)));

  public static final RegistryObject<Item> PINE_CONE =
      ITEMS.register("pine_cone", () -> new FuelItem(new Item.Properties(), 999));

  public static final RegistryObject<Item> SAPPHIRE_SWORD =
      ITEMS.register(
          "sapphire_sword",
          () -> new SwordItem(ModToolTiers.SAPPHIRE, 999, 999, new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE_PICKAXE =
      ITEMS.register(
          "sapphire_pickaxe",
          () -> new PickaxeItem(ModToolTiers.SAPPHIRE, 999, 999, new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE_AXE =
      ITEMS.register(
          "sapphire_axe",
          () -> new AxeItem(ModToolTiers.SAPPHIRE, 999, 999, new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE_SHOVEL =
      ITEMS.register(
          "sapphire_shovel",
          () -> new ShovelItem(ModToolTiers.SAPPHIRE, 999, 999, new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE_HOE =
      ITEMS.register(
          "sapphire_hoe",
          () -> new HoeItem(ModToolTiers.SAPPHIRE, 999, 999, new Item.Properties()));

  public static final RegistryObject<Item> SAPPHIRE_HELMET =
      ITEMS.register(
          "sapphire_helmet",
          () ->
              new ModArmorItem(
                  ModArmorMaterials.SAPPHIRE, ArmorItem.Type.HELMET, new Item.Properties()));
  public static final RegistryObject<Item> SAPPHIRE_CHESTPLATE =
      ITEMS.register(
          "sapphire_chestplate",
          () ->
              new ArmorItem(
                  ModArmorMaterials.SAPPHIRE, ArmorItem.Type.CHESTPLATE, new Item.Properties()));
  public static final RegistryObject<Item> SAPPHIRE_LEGGINGS =
      ITEMS.register(
          "sapphire_leggings",
          () ->
              new ArmorItem(
                  ModArmorMaterials.SAPPHIRE, ArmorItem.Type.LEGGINGS, new Item.Properties()));
  public static final RegistryObject<Item> SAPPHIRE_BOOTS =
      ITEMS.register(
          "sapphire_boots",
          () ->
              new ArmorItem(
                  ModArmorMaterials.SAPPHIRE, ArmorItem.Type.BOOTS, new Item.Properties()));

  public static final RegistryObject<Item> STRAWBERRY_SEEDS =
      ITEMS.register(
          "strawberry_seeds",
          () -> new ItemNameBlockItem(ModBlocks.STRAWBERRY_CROP.get(), new Item.Properties()));

  public static final RegistryObject<Item> CORN_SEEDS =
      ITEMS.register(
          "corn_seeds",
          () -> new ItemNameBlockItem(ModBlocks.CORN_CROP.get(), new Item.Properties()));
  public static final RegistryObject<Item> CORN =
      ITEMS.register("corn", () -> new Item(new Item.Properties().food(ModFoods.CORN)));
  public static final RegistryObject<Item> BAR_BRAWL_MUSIC_DISC =
      ITEMS.register(
          "bar_brawl_music_disc",
          () -> new RecordItem(6, ModSounds.BAR_BRAWL, new Item.Properties().stacksTo(1), 2440));

  public static void register(IEventBus modEventBus) {
    ITEMS.register(modEventBus);
  }
}

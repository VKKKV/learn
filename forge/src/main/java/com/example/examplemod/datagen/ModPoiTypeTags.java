package com.example.examplemod.datagen;

import java.util.concurrent.CompletableFuture;
import net.minecraft.core.HolderLookup.Provider;
import net.minecraft.data.PackOutput;
import net.minecraft.data.tags.PoiTypeTagsProvider;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.tags.PoiTypeTags;
import net.minecraftforge.common.data.ExistingFileHelper;
import org.jetbrains.annotations.Nullable;

public class ModPoiTypeTags extends PoiTypeTagsProvider {

  private String modId;

  public ModPoiTypeTags(
      PackOutput pOutput,
      CompletableFuture<Provider> pProvider,
      String modId,
      @Nullable ExistingFileHelper existingFileHelper) {
    super(pOutput, pProvider, modId, existingFileHelper);
    this.modId = modId;
  }

  @Override
  protected void addTags(Provider pProvider) {
    tag(PoiTypeTags.ACQUIRABLE_JOB_SITE)
        .addOptional(ResourceLocation.fromNamespaceAndPath(this.modId, "sound_poi"));
  }
}
